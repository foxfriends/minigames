use crate::guild::GuildId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgConnection;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct GameId(Uuid);

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct Game {
    pub id: GameId,
    pub guild_id: GuildId,
    pub game: String,
    pub state: Value,
}

impl Game {
    pub async fn create(
        guild_id: GuildId,
        game: &str,
        conn: &mut PgConnection,
    ) -> anyhow::Result<Self> {
        let game = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO games (guild_id, game)
                VALUES ($1, $2)
            RETURNING
                id as "id: _",
                guild_id as "guild_id: _",
                game,
                state
                "#,
            guild_id as GuildId,
            game
        )
        .fetch_one(conn)
        .await?;
        Ok(game)
    }
}
