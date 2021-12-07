use super::Claims;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token(pub(super) String);

impl Token {
    pub fn new(contents: String) -> Self {
        Self(contents)
    }

    pub fn decode(&self) -> anyhow::Result<Claims> {
        Claims::decode(self)
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
