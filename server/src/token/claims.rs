use super::jwt;
use super::Token;
use crate::game::GameName;
use crate::user::UserId;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The URL of the main server.
    iss: String,
    /// The game for which this token is valid.
    aud: GameName,
    /// The user's Discord ID.
    sub: UserId,
    /// Issued at.
    iat: u64,
}

impl Claims {
    pub fn new(game_name: GameName, user_id: UserId) -> Self {
        Claims {
            iss: crate::env::public_http_url(),
            aud: game_name,
            sub: user_id,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn sign(&self) -> anyhow::Result<Token> {
        jwt::sign(self)
    }

    pub fn verify(token: Token) -> anyhow::Result<Claims> {
        jwt::verify(token)
    }

    pub fn is_game(&self, game: &GameName) -> bool {
        &self.aud == game
    }

    pub fn user_id(&self) -> UserId {
        self.sub
    }
}
