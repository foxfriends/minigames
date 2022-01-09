use crate::discord;
use crate::discord::DiscordUser;

pub struct DashboardContext {
    pub path: Vec<String>,
    pub user: DiscordUser,
}

impl DashboardContext {
    pub async fn load<P>(path: P, discord_user_token: &str) -> anyhow::Result<Self>
    where
        P: IntoIterator,
        P::Item: Into<String>,
    {
        let path = path.into_iter().map(Into::into).collect();
        let user = discord::get_current_user(discord_user_token).await?;
        Ok(Self { path, user })
    }

    pub fn section(&self) -> &str {
        self.path.first().unwrap()
    }

    pub fn title(&self) -> &str {
        self.path.last().unwrap()
    }
}
