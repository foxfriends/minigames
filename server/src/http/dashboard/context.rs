use crate::discord;
use crate::discord::{DiscordGuild, DiscordUser, UserDiscordGuild};
use crate::game::{GameName, GameRegistry};
use crate::guild::GuildId;

pub struct DashboardContextBuilder {
    token: String,
    user: DiscordUser,
    guild: Option<DiscordGuild>,
    path: Vec<String>,
    registry: Option<GameRegistry>,
}

impl DashboardContextBuilder {
    pub fn with_registry(mut self, registry: GameRegistry) -> Self {
        self.registry = Some(registry);
        self
    }

    pub fn with_path<P>(mut self, path: P) -> Self
    where
        P: IntoIterator,
        P::Item: Into<String>,
    {
        self.path.extend(path.into_iter().map(Into::into));
        self
    }

    pub async fn with_guild(mut self, guild_id: GuildId) -> anyhow::Result<Self> {
        let guild = discord::get_guild(guild_id).await?;
        self.path.push(guild.name.clone());
        self.guild = Some(guild);
        Ok(self)
    }

    pub fn build(self) -> DashboardContext {
        DashboardContext {
            token: self.token,
            path: self.path,
            user: self.user,
            guild: self.guild,
            registry: self.registry,
        }
    }
}

pub struct DashboardContext {
    path: Vec<String>,
    token: String,
    user: DiscordUser,
    guild: Option<DiscordGuild>,
    registry: Option<GameRegistry>,
}

impl DashboardContext {
    pub async fn builder(token: &str) -> anyhow::Result<DashboardContextBuilder> {
        let user = discord::get_current_user(token).await?;
        Ok(DashboardContextBuilder {
            path: vec![],
            token: token.to_owned(),
            user,
            guild: None,
            registry: None,
        })
    }

    pub fn section(&self) -> &str {
        self.path.first().unwrap()
    }

    pub fn title(&self) -> &str {
        self.path.last().unwrap()
    }

    pub fn user(&self) -> &DiscordUser {
        &self.user
    }

    pub async fn load_guilds(&self) -> anyhow::Result<Vec<UserDiscordGuild>> {
        discord::get_user_guilds(&self.token).await
    }

    pub async fn is_available(&self, game: &GameName) -> bool {
        match &self.registry {
            Some(registry) => registry.is_available(game).await,
            None => true,
        }
    }
}
