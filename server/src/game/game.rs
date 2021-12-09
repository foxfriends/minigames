use super::GameId;
use super::GameName;
use super::GameState;
use crate::guild::GuildId;
use sqlx::postgres::PgConnection;

pub struct Game {
    pub id: GameId,
    pub guild_id: GuildId,
    pub game: GameName,
    pub state: GameState,
}

impl Game {
    pub async fn create(
        guild_id: GuildId,
        game: GameName,
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
                game as "game: _",
                state as "state: _"
                "#,
            guild_id as GuildId,
            game as GameName,
        )
        .fetch_one(conn)
        .await?;
        Ok(game)
    }

    pub async fn load(game_id: GameId, conn: &mut PgConnection) -> anyhow::Result<Self> {
        let game = sqlx::query_as!(
            Self,
            r#"
            SELECT
                id as "id: _",
                guild_id as "guild_id: _",
                game as "game: _",
                state as "state: _"
            FROM games
            WHERE id = $1
            "#,
            game_id as GameId
        )
        .fetch_one(conn)
        .await?;
        Ok(game)
    }

    pub async fn update(
        game_id: GameId,
        state: GameState,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE games SET state = $1 WHERE id = $2"#,
            state as GameState,
            game_id as GameId,
        )
        .execute(conn)
        .await?;
        Ok(())
    }
}
