use crate::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameParticipant {
    pub user_id: UserId,
    pub is_challenger: bool,
}
