use rocket::form::{self, FromFormField, ValueField};
use rocket::request::FromParam;
use rocket::UriDisplayPath;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, sqlx::Type, UriDisplayPath)]
#[sqlx(transparent)]
pub struct GameName(String);

impl GameName {
    pub fn initials(&self) -> String {
        self.0
            .split(' ')
            .filter_map(|word| word.chars().next())
            .collect()
    }
}

impl Display for GameName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl std::ops::Deref for GameName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> FromParam<'a> for GameName {
    type Error = Infallible;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(param.to_owned()))
    }
}

impl<'r> FromFormField<'r> for GameName {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self(field.value.to_owned()))
    }
}
