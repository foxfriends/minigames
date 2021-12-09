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
