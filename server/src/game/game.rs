use super::{GameId, GameName, GameParticipant, GameState};
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

    pub async fn participants(
        &self,
        conn: &mut PgConnection,
    ) -> anyhow::Result<Vec<GameParticipant>> {
        Ok(sqlx::query_as!(
            GameParticipant,
            r#"SELECT user_id as "id: UserId", is_challenger FROM game_participants WHERE game_id = $1"#,
            self.id as GameId,
        )
        .fetch_all(conn)
        .await?)
    }

    pub async fn vote_result(
        &self,
        voter_id: UserId,
        winner_id: Option<UserId>,
        conn: &mut PgConnection,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO game_complete_votes (game_id, user_id, winner_id) VALUES ($1, $2, $3) ON CONFLICT (game_id, user_id) DO UPDATE SET winner_id = $3",
            self.id as GameId,
            voter_id as UserId,
            winner_id as Option<UserId>,
        ).execute(conn).await?;
        Ok(())
    }

    pub async fn check_winner<Conn>(&self, mut conn: Conn) -> anyhow::Result<Option<Option<UserId>>>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let participants = sqlx::query!(
            "SELECT count(user_id) as count FROM game_participants WHERE game_id = $1",
            self.id as GameId
        )
        .fetch_one(&mut *conn)
        .await?
        .count
        .unwrap();
        let votes = sqlx::query!(
            r#"SELECT user_id, winner_id as "winner_id: UserId" FROM game_complete_votes WHERE game_id = $1"#,
            self.id as GameId
        )
        .fetch_all(&mut *conn)
        .await?;
        if participants as usize != votes.len() {
            Ok(None)
        } else {
            Ok(Some(votes[0].winner_id))
        }
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
