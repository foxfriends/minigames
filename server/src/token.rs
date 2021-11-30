use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    /// The Discord server (guild) in which this challenge is issued.
    iss: String,
    /// The type of challenge this token represents.
    aud: String,
    /// The time by which this challenge must be accepted.
    exp: usize,
    /// The database ID which stores the relevant challenge data.
    sub: String,
}
