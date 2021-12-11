use super::Token;
use crate::game::GameName;
use crate::key::{JWT_KEY, SPKI};
use crate::user::UserId;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static::lazy_static! {
    static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_rsa_pem(&JWT_KEY).unwrap();
    static ref JWT_DECODING_KEY: DecodingKey<'static> = DecodingKey::from_rsa_pem(SPKI.as_bytes()).unwrap();
}

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
            iss: env::var("PUBLIC_HTTP_URL").unwrap(),
            aud: game_name,
            sub: user_id,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn sign(&self) -> anyhow::Result<Token> {
        Ok(Token(encode(
            &Header::new(Algorithm::RS256),
            self,
            &JWT_ENCODING_KEY,
        )?))
    }

    pub fn verify(token: Token) -> anyhow::Result<Claims> {
        let validation = Validation {
            validate_exp: false,
            iss: Some(env::var("PUBLIC_HTTP_URL").unwrap()),
            algorithms: vec![Algorithm::RS256],
            ..Validation::default()
        };
        Ok(decode(&token.0, &JWT_DECODING_KEY, &validation)?.claims)
    }

    pub fn is_game(&self, game: &GameName) -> bool {
        &self.aud == game
    }

    pub fn user_id(&self) -> UserId {
        self.sub
    }
}
