use crate::user::UserId;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: UserId,
    pub username: String,
}

pub async fn get_current_user(bearer: String) -> anyhow::Result<DiscordUser> {
    Ok(Client::new()
        .request(Method::GET, "https://discord.com/api/users/@me")
        .bearer_auth(bearer)
        .send()
        .await?
        .json()
        .await?)
}
