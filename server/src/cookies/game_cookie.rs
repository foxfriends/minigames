use crate::game::GameId;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::request::{self, FromRequest, Outcome, Request};
use std::str::FromStr;
use time::Duration;
use uuid::Uuid;

const GAME_TOKEN_COOKIE: &str = "minigames-game-token";

#[derive(Debug)]
pub struct GameCookie(GameId);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GameCookie {
    type Error = <Uuid as FromStr>::Err;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.cookies().get(GAME_TOKEN_COOKIE) {
            Some(cookie) => match cookie.value().parse() {
                Ok(value) => Outcome::Success(Self(GameId::new(value))),
                Err(error) => Outcome::Failure((Status::BadRequest, error)),
            },
            None => Outcome::Forward(()),
        }
    }
}

impl GameCookie {
    pub fn value(&self) -> GameId {
        self.0
    }

    pub fn add_to(cookie_jar: &CookieJar, value: GameId) {
        let mut cookie = Cookie::new(GAME_TOKEN_COOKIE, value.to_string());
        cookie.set_http_only(true);
        cookie.set_same_site(Some(SameSite::Lax));
        cookie.set_max_age(Some(Duration::seconds(60 * 5)));
        cookie_jar.add(cookie);
    }

    pub fn remove_from(cookie_jar: &CookieJar) {
        cookie_jar.remove(Cookie::named(GAME_TOKEN_COOKIE));
    }
}
