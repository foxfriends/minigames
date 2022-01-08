use crate::user::UserId;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordUser {
    pub id: UserId,
    pub discriminator: String,
    pub username: String,
    pub avatar: String,
}

impl DiscordUser {
    pub fn avatar_url(&self, size: u32) -> String {
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png?size={}",
            self.id,
            self.avatar,
            2u8.pow(size)
        )
    }
}

pub async fn get_current_user(bearer: &str) -> anyhow::Result<DiscordUser> {
    Ok(Client::new()
        .request(Method::GET, "https://discord.com/api/users/@me")
        .bearer_auth(bearer)
        .send()
        .await?
        .json()
        .await?)
}
