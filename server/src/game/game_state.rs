use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct GameState(Value);
