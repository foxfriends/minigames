use crate::discord;
use crate::discord::DiscordUser;

pub struct DashboardContext {
    pub title: String,
    pub user: DiscordUser,
}

impl DashboardContext {
    pub async fn load<S: Into<String>>(title: S, discord_user_token: &str) -> anyhow::Result<Self> {
        let user = discord::get_current_user(discord_user_token).await?;
        Ok(Self {
            title: title.into(),
            user,
        })
    }
}
