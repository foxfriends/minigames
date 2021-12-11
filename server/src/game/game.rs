use super::GameId;
use super::GameName;
use super::GameState;
use crate::guild::GuildId;
use crate::user::{User, UserId};
use sqlx::postgres::{PgConnection, Postgres};
use sqlx::Executor;

pub struct Game {
    pub id: GameId,
    pub guild_id: GuildId,
    pub game: GameName,
    pub state: GameState,
}

impl Game {
    pub async fn create<Conn>(
        guild_id: GuildId,
        game: GameName,
        challenger: User,
        mut conn: Conn,
    ) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
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
        .fetch_one(&mut *conn)
        .await?;
        sqlx::query!(
            "INSERT INTO game_participants (game_id, user_id, is_challenger) VALUES ($1, $2, true)",
            game.id as GameId,
            challenger.id as UserId,
        )
        .execute(&mut *conn)
        .await?;
        Ok(game)
    }

    pub async fn add_participant(&self, user: User, conn: &mut PgConnection) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO game_participants (game_id, user_id) VALUES ($1, $2)",
            self.id as GameId,
            user.id as UserId,
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    pub async fn is_participant(
        &self,
        user: UserId,
        conn: &mut PgConnection,
    ) -> anyhow::Result<bool> {
        Ok(sqlx::query!(
            "SELECT EXISTS (SELECT 1 FROM game_participants WHERE game_id = $1 AND user_id = $2)",
            self.id as GameId,
            user as UserId,
        )
        .fetch_one(conn)
        .await?
        .exists
        .unwrap_or(false))
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
