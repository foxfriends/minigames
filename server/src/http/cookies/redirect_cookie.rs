use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::request::{self, FromRequest, Outcome, Request};
use time::Duration;

const REDIRECT_COOKIE_NAME: &str = "minigames-redirect";

#[derive(Debug)]
pub struct RedirectCookie(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RedirectCookie {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get(REDIRECT_COOKIE_NAME) {
            Some(cookie) => Outcome::Success(Self(cookie.value().to_owned())),
            None => Outcome::Forward(()),
        }
    }
}

impl RedirectCookie {
    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn add_to(cookie_jar: &CookieJar, value: String) {
        let mut cookie = Cookie::new(REDIRECT_COOKIE_NAME, value);
        cookie.set_http_only(true);
        cookie.set_same_site(Some(SameSite::Lax));
        cookie.set_max_age(Some(Duration::seconds(60 * 5)));
        cookie_jar.add(cookie);
    }

    pub fn remove_from(cookie_jar: &CookieJar) {
        cookie_jar.remove(Cookie::named(REDIRECT_COOKIE_NAME));
    }
}
