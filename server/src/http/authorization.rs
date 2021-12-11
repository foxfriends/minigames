use crate::token::{Claims, Token};
use crate::user::UserId;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct UserAuthorization(Claims);

impl UserAuthorization {
    pub fn user_id(&self) -> UserId {
        self.0.user_id()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuthorization {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get("Authorization").next() {
            Some(bearer) if bearer.starts_with("Bearer ") => {
                let token = &bearer[7..];
                match Claims::verify(Token::from(token)) {
                    Ok(claims) => Outcome::Success(Self(claims)),
                    Err(..) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            Some(..) => Outcome::Failure((Status::Unauthorized, ())),
            None => Outcome::Forward(()),
        }
    }
}
