use crate::env::discord_bot_token;
use crate::guild::GuildId;
use crate::user::UserId;
use num_bigint::BigUint;
use reqwest::{Client, Method};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
        .request(Method::GET, "https://discord.com/api/v8/users/@me")
        .bearer_auth(bearer)
        .send()
        .await?
        .json()
        .await?)
}

#[derive(Debug)]
pub struct DiscordPermissions(BigUint);

impl<'de> Deserialize<'de> for DiscordPermissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = <&str>::deserialize(deserializer)?;
        Ok(DiscordPermissions(
            string.parse().map_err(de::Error::custom)?,
        ))
    }
}

impl DiscordPermissions {
    fn has_permission(&self, other: &Self) -> bool {
        &self.0 & &other.0 != BigUint::from(0u32)
    }

    fn manage_guild() -> Self {
        Self(BigUint::from(0x20u32))
    }
}

#[derive(Debug, Deserialize)]
pub struct UserDiscordGuild {
    pub id: GuildId,
    pub name: String,
    pub icon: Option<String>,
    pub permissions: DiscordPermissions,
}

impl UserDiscordGuild {
    pub fn icon_url(&self, size: u32) -> Option<String> {
        Some(format!(
            "https://cdn.discordapp.com/icons/{}/{}.png?size={}",
            self.id,
            self.icon.as_ref()?,
            2u8.pow(size)
        ))
    }

    pub fn initials(&self) -> String {
        self.name
            .split(' ')
            .filter_map(|word| word.chars().next())
            .collect()
    }

    pub fn can_manage(&self) -> bool {
        self.permissions
            .has_permission(&DiscordPermissions::manage_guild())
    }
}

pub async fn get_user_guilds(bearer: &str) -> anyhow::Result<Vec<UserDiscordGuild>> {
    let bot_guilds = get_bot_guilds()
        .await?
        .into_iter()
        .map(|guild| guild.id)
        .collect::<HashSet<_>>();
    let user_guilds: Vec<UserDiscordGuild> = Client::new()
        .request(Method::GET, "https://discord.com/api/v8/users/@me/guilds")
        .bearer_auth(bearer)
        .send()
        .await?
        .json()
        .await?;
    Ok(user_guilds
        .into_iter()
        .filter(|guild| bot_guilds.contains(&guild.id))
        .collect())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordGuild {
    pub id: GuildId,
    pub name: String,
    pub icon: Option<String>,
}

pub async fn get_bot_guilds() -> anyhow::Result<Vec<DiscordGuild>> {
    Ok(Client::new()
        .request(Method::GET, "https://discord.com/api/v8/users/@me/guilds")
        .header("Authorization", format!("Bot {}", discord_bot_token()))
        .send()
        .await?
        .json()
        .await?)
}

pub async fn get_guild(guild_id: GuildId) -> anyhow::Result<DiscordGuild> {
    Ok(Client::new()
        .request(
            Method::GET,
            format!("https://discord.com/api/v8/guilds/{}", guild_id),
        )
        .header("Authorization", format!("Bot {}", discord_bot_token()))
        .send()
        .await?
        .json()
        .await?)
}

pub async fn get_user(user_id: UserId) -> anyhow::Result<DiscordUser> {
    Ok(Client::new()
        .request(
            Method::GET,
            format!("https://discord.com/api/v8/users/{}", user_id),
        )
        .header("Authorization", format!("Bot {}", discord_bot_token()))
        .send()
        .await?
        .json()
        .await?)
}
