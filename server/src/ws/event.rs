use crate::game::{GameId, GameState};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Formatter};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct EventId(String);

#[derive(Clone, Serialize, Deserialize)]
pub struct Event<T> {
    pub id: EventId,
    pub payload: T,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Action {
    Subscribe(GameId),
    Unsubscribe(GameId),
    Get(GameId),
    Set(GameId, GameState),
}

impl Debug for Action {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Subscribe(id) => write!(f, "Subscribe({:?})", id),
            Self::Unsubscribe(id) => write!(f, "Unsubscribe({:?})", id),
            Self::Get(id) => write!(f, "Get({:?})", id),
            Self::Set(id, ..) => write!(f, "Set({:?}, {{ ... }})", id),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Response {
    Update(GameState),
    Error(String), // TODO: error type? with code + context?
}
