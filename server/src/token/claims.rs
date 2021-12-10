use super::Token;
use crate::game::GameName;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;

lazy_static::lazy_static! {
    static ref JWT_ENCODING_KEY: EncodingKey = {
        let key_path = env::var("JWT_PEM").unwrap();
        let mut file = File::open(key_path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        EncodingKey::from_ec_pem(&buf).unwrap()
    };
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
            &Header::new(Algorithm::ES256),
            self,
            &JWT_ENCODING_KEY,
        )?))
    }
}
