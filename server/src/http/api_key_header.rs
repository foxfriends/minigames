use crate::game::ApiKey;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

pub struct ApiKeyHeader(ApiKey);

impl ApiKeyHeader {
    pub fn value(&self) -> &ApiKey {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyHeader {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get("X-Api-Key").next() {
            Some(api_key_b64) => match api_key_b64.parse() {
                Ok(api_key) => Outcome::Success(ApiKeyHeader(api_key)),
                Err(error) => Outcome::Failure((Status::BadRequest, error)),
            },
            None => Outcome::Forward(()),
        }
    }
}
