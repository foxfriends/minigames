use rocket::request::{self, FromRequest, Outcome, Request};
use std::ops::Deref;

pub struct Lax<T>(pub T);

#[rocket::async_trait]
impl<'r, T> FromRequest<'r> for Lax<T>
where
    T: FromRequest<'r>,
{
    type Error = std::convert::Infallible;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match T::from_request(req).await {
            Outcome::Failure(..) => Outcome::Forward(()),
            Outcome::Forward(()) => Outcome::Forward(()),
            Outcome::Success(value) => Outcome::Success(Lax(value)),
        }
    }
}

impl<T> Deref for Lax<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
