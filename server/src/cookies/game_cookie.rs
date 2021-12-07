use crate::token::Token;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::request::{self, FromRequest, Outcome, Request};
use time::Duration;

const GAME_TOKEN_COOKIE: &str = "minigames-game-token";

#[derive(Debug)]
pub struct GameCookie(Token);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GameCookie {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get(GAME_TOKEN_COOKIE) {
            Some(cookie) => Outcome::Success(Self(Token::new(cookie.value().to_owned()))),
            None => Outcome::Forward(()),
        }
    }
}

impl GameCookie {
    pub fn value(&self) -> &Token {
        &self.0
    }

    pub fn add_to(cookie_jar: &CookieJar, value: Token) {
        let mut cookie = Cookie::new(GAME_TOKEN_COOKIE, value.to_string());
        cookie.set_http_only(true);
        cookie.set_same_site(Some(SameSite::Strict));
        cookie.set_max_age(Some(Duration::seconds(60 * 5)));
        cookie_jar.add(cookie);
    }

    pub fn remove_from(cookie_jar: &CookieJar) {
        cookie_jar.remove(Cookie::named(GAME_TOKEN_COOKIE));
    }
}
