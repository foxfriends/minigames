use rocket::form::{self, Errors, FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct GameId(Uuid);

impl GameId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

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
