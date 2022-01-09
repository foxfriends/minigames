use rocket::request::FromParam;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct GameName(String);

impl Display for GameName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
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
