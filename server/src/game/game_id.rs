use rocket::form::{self, Errors, FromFormField, ValueField};
use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct GameId(Uuid);

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'r> FromFormField<'r> for GameId {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match field.value.parse() {
            Ok(value) => Ok(Self(value)),
            Err(..) => Err(Errors::from(vec![field.unexpected()])),
        }
    }
}

impl<'a> FromParam<'a> for GameId {
    type Error = <Uuid as FromStr>::Err;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(param.parse()?))
    }
}
