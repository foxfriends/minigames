use super::jwt;
use super::Token;
use crate::game::GameName;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckClaims {
    /// The URL of the main server.
    iss: String,
    /// The game for which this token is valid.
    aud: GameName,
    /// Issued at.
    iat: u64,
}

impl HealthCheckClaims {
    pub fn new(game_name: GameName) -> Self {
        HealthCheckClaims {
            iss: crate::env::public_http_url(),
            aud: game_name,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn sign(&self) -> anyhow::Result<Token> {
        jwt::sign(self)
    }
}
