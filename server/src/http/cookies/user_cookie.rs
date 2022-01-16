use oauth2::{TokenResponse, TokenType};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::request::{self, FromRequest, Outcome, Request};
use time::Duration;

const DISCORD_USER_COOKIE: &str = "minigames-discord-user";

#[derive(Debug)]
pub struct UserCookie<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserCookie<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get(DISCORD_USER_COOKIE) {
            Some(cookie) => Outcome::Success(Self(cookie.value())),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

impl<'r> UserCookie<'r> {
    pub fn value(&self) -> &'r str {
        self.0
    }

    pub fn add_to<TT: TokenType, T: TokenResponse<TT> + 'static>(
        cookie_jar: &CookieJar,
        token: &T,
    ) {
        let secret = token.access_token().secret();
        let mut cookie = Cookie::new(DISCORD_USER_COOKIE, secret.to_owned());
        cookie.set_http_only(true);
        cookie.set_same_site(Some(SameSite::Lax));
        cookie.set_max_age(
            token
                .expires_in()
                .map(|dur| Duration::seconds(dur.as_secs() as i64)),
        );
        cookie_jar.add(cookie);
    }

    pub fn remove_from(cookie_jar: &CookieJar) {
        cookie_jar.remove(Cookie::named(DISCORD_USER_COOKIE));
    }
}
