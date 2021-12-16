use crate::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameParticipant {
    pub id: UserId,
    pub is_challenger: bool,
}
