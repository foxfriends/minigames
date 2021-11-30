use crate::game::GameId;
use crate::guild::GuildId;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The Discord server (guild) in which this challenge is issued.
    iss: String,
    /// The type of challenge this token represents.
    aud: String,
    /// The time by which this challenge must be accepted.
    exp: usize,
    /// The database ID which stores the relevant challenge data.
    sub: String,
}

impl Claims {
    pub fn sign(&self) -> anyhow::Result<Token> {
        Ok(Token(encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()),
        )?))
    }

    #[allow(dead_code)]
    pub fn decode(token: &Token) -> anyhow::Result<Self> {
        Ok(decode(
            &token.0,
            &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()),
            &Validation::default(),
        )?
        .claims)
    }

    pub fn new_challenge(guild_id: GuildId, game_id: GameId) -> Self {
        Claims {
            iss: guild_id.to_string(),
            aud: String::from("challenge"),
            exp: (SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
                + Duration::from_secs(60 * 15))
            .as_secs() as usize,
            sub: game_id.to_string(),
        }
    }
}
