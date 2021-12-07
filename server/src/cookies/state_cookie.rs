use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::request::{self, FromRequest, Outcome, Request};
use time::Duration;

const OAUTH_STATE_COOKIE: &str = "minigames-discord-auth-state";

#[derive(Debug)]
pub struct StateCookie<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StateCookie<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get(OAUTH_STATE_COOKIE) {
            Some(cookie) => Outcome::Success(Self(cookie.value())),
            None => Outcome::Forward(()),
        }
    }
}

impl<'r> StateCookie<'r> {
    pub fn value(&self) -> &'r str {
        self.0
    }

    pub fn add_to(cookie_jar: &CookieJar, value: String) {
        let mut cookie = Cookie::new(OAUTH_STATE_COOKIE, value);
        cookie.set_http_only(true);
        cookie.set_secure(true);
        cookie.set_same_site(Some(SameSite::Lax));
        cookie.set_max_age(Some(Duration::seconds(60 * 5)));
        cookie_jar.add(cookie);
    }

    pub fn remove_from(cookie_jar: &CookieJar) {
        cookie_jar.remove(Cookie::named(OAUTH_STATE_COOKIE));
    }
}
