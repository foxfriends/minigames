use super::Token;
use crate::key::{JWT_KEY, SPKI};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    static ref JWT_ENCODING_KEY: EncodingKey = EncodingKey::from_rsa_pem(&JWT_KEY).unwrap();
    static ref JWT_DECODING_KEY: DecodingKey<'static> = DecodingKey::from_rsa_pem(SPKI.as_bytes()).unwrap();
}

pub fn sign<Claims: Serialize>(claims: &Claims) -> anyhow::Result<Token> {
    Ok(Token(encode(
        &Header::new(Algorithm::RS256),
        claims,
        &JWT_ENCODING_KEY,
    )?))
}

pub fn verify<Claims: for<'de> Deserialize<'de>>(token: Token) -> anyhow::Result<Claims> {
    let validation = Validation {
        validate_exp: false,
        iss: Some(crate::env::public_http_url()),
        algorithms: vec![Algorithm::RS256],
        ..Validation::default()
    };
    Ok(decode(&token.0, &JWT_DECODING_KEY, &validation)?.claims)
}
