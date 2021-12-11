use super::Token;
use crate::game::GameName;
use crate::key::JWT_KEY;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

lazy_static::lazy_static! {
    static ref JWT_ENCODING_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(&JWT_KEY).unwrap()
    ;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The URL of the main server.
    iss: String,
    /// The game for which this token is valid.
    aud: GameName,
    /// The user's Discord ID.
    sub: String,
}

impl Claims {
    pub fn new(game_name: GameName, user_id: String) -> Self {
        Claims {
            iss: env::var("PUBLIC_HTTP_URL").unwrap(),
            aud: game_name,
            sub: user_id,
        }
    }

    pub fn sign(&self) -> anyhow::Result<Token> {
        Ok(Token(encode(
            &Header::new(Algorithm::RS256),
            self,
            &JWT_ENCODING_KEY,
        )?))
    }
}
