use reqwest::header::HeaderValue;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token(pub(super) String);

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'r> FromFormField<'r> for Token {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self(field.value.to_owned()))
    }
}

impl From<Token> for HeaderValue {
    fn from(token: Token) -> Self {
        Self::try_from(&token.0).unwrap()
    }
}
