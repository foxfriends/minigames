use crate::game::{GameId, GameState};
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize)]
pub enum Response {
    Update(GameState),
    Error(String), // TODO: error type? with code + context?
}
