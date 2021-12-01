use super::Claims;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(pub(super) String);

impl Token {
    pub fn decode(&self) -> anyhow::Result<Claims> {
        Claims::decode(self)
    }
}

impl<'r> FromFormField<'r> for Token {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self(field.value.to_owned()))
    }
}
