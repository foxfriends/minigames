use crate::discord;
use crate::discord::DiscordUser;
use crate::game::{GameName, GameRegistry};

pub struct DashboardContextBuilder {
    path: Vec<String>,
    user: Option<DiscordUser>,
    registry: Option<GameRegistry>,
}

impl DashboardContextBuilder {
    pub fn with_registry(mut self, registry: GameRegistry) -> Self {
        self.registry = Some(registry);
        self
    }

    pub async fn load_user(mut self, discord_user_token: &str) -> anyhow::Result<Self> {
        let user = discord::get_current_user(discord_user_token).await?;
        self.user = Some(user);
        Ok(self)
    }

    pub fn build(self) -> DashboardContext {
        DashboardContext {
            path: self.path,
            user: self.user,
            registry: self.registry,
        }
    }
}

pub struct DashboardContext {
    path: Vec<String>,
    user: Option<DiscordUser>,
    registry: Option<GameRegistry>,
}

impl DashboardContext {
    pub fn builder<P>(path: P) -> DashboardContextBuilder
    where
        P: IntoIterator,
        P::Item: Into<String>,
    {
        DashboardContextBuilder {
            path: path.into_iter().map(Into::into).collect(),
            user: None,
            registry: None,
        }
    }

    pub fn section(&self) -> &str {
        self.path.first().unwrap()
    }

    pub fn title(&self) -> &str {
        self.path.last().unwrap()
    }

    pub fn user(&self) -> &DiscordUser {
        self.user.as_ref().unwrap()
    }

    pub async fn is_available(&self, game: &GameName) -> bool {
        match &self.registry {
            Some(registry) => registry.is_available(game).await,
            None => true,
        }
    }
}
