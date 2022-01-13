use super::{ApiKeys, GameName};
use crate::env::superuser_id;
use crate::guild::{Guild, GuildId};
use crate::user::UserId;
use serde::{Deserialize, Serialize};
use sqlx::postgres::Postgres;
use sqlx::Executor;

#[derive(Serialize, Deserialize)]
pub struct GameServer {
    name: GameName,
    user_id: UserId,
    pub public_url: String,
    pub enabled: bool,
}

impl GameServer {
    pub fn name(&self) -> &GameName {
        &self.name
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub async fn create<Conn>(
        name: &GameName,
        user_id: UserId,
        public_url: &str,
        mut conn: Conn,
    ) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let game_server = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO game_servers (name, user_id, public_url)
                VALUES ($1, $2, $3)
                RETURNING
                    name as "name: _",
                    user_id as "user_id: _",
                    public_url,
                    enabled
            "#,
            name as &GameName,
            user_id as UserId,
            public_url,
        )
        .fetch_one(&mut *conn)
        .await?;
        ApiKeys::generate(name, &mut *conn).await?;
        Ok(game_server)
    }

    pub async fn set_guilds<Conn>(&self, guilds: &[GuildId], mut conn: Conn) -> anyhow::Result<()>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        sqlx::query!(
            "DELETE FROM game_server_guilds WHERE game_server_name = $1",
            &self.name as &GameName
        )
        .execute(&mut *conn)
        .await?;
        for guild_id in guilds {
            Guild::upsert(guild_id, &mut *conn).await?;
            sqlx::query!(
                "INSERT INTO game_server_guilds (game_server_name, guild_id) VALUES ($1, $2)",
                &self.name as &GameName,
                guild_id as &GuildId
            )
            .execute(&mut *conn)
            .await?;
        }
        Ok(())
    }

    pub async fn is_in_guild<Conn>(&self, guild_id: GuildId, mut conn: Conn) -> anyhow::Result<bool>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        if superuser_id().map(|id| id == self.user_id).unwrap_or(false) {
            return Ok(true);
        }
        let result = sqlx::query!("SELECT EXISTS (SELECT 1 FROM game_server_guilds WHERE game_server_name = $1 AND guild_id = $2)", &self.name as &GameName, guild_id as GuildId)
            .fetch_one(&mut *conn)
            .await?;
        Ok(result.exists.unwrap())
    }

    pub async fn list_all<Conn>(mut conn: Conn) -> anyhow::Result<Vec<Self>>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let servers = sqlx::query_as!(
            Self,
            r#"
            SELECT
                name as "name: _",
                user_id as "user_id: _",
                public_url,
                enabled
            FROM game_servers
            ORDER BY name ASC
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;
        Ok(servers)
    }

    pub async fn list_all_for_guild<Conn>(
        guild_id: &GuildId,
        mut conn: Conn,
    ) -> anyhow::Result<Vec<Self>>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let servers = match superuser_id() {
            Some(superuser) => {
                sqlx::query_as!(
                    Self,
                    r#"
                    SELECT DISTINCT
                        name as "name: _",
                        user_id as "user_id: _",
                        public_url,
                        enabled
                    FROM game_servers g
                    LEFT OUTER JOIN game_server_guilds l ON l.game_server_name = g.name
                    WHERE user_id = $1 OR l.guild_id = $2
                    ORDER BY name ASC
                    "#,
                    superuser as UserId,
                    guild_id as &GuildId,
                )
                .fetch_all(&mut *conn)
                .await?
            }
            None => {
                sqlx::query_as!(
                    Self,
                    r#"
                SELECT
                    name as "name: _",
                    user_id as "user_id: _",
                    public_url,
                    enabled
                FROM game_servers g
                INNER JOIN game_server_guilds l ON g.name = l.game_server_name
                WHERE l.guild_id = $1
                ORDER BY name ASC
                "#,
                    guild_id as &GuildId,
                )
                .fetch_all(&mut *conn)
                .await?
            }
        };
        Ok(servers)
    }

    pub async fn load<Conn>(name: &GameName, mut conn: Conn) -> anyhow::Result<Option<Self>>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let server = sqlx::query_as!(
            Self,
            r#"
            SELECT
                name as "name: _",
                user_id as "user_id: _",
                public_url,
                enabled
            FROM game_servers
            WHERE name = $1
            "#,
            name as &GameName,
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(server)
    }

    pub async fn save<Conn>(&self, mut conn: Conn) -> anyhow::Result<()>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            UPDATE game_servers
            SET public_url = $1,
                enabled = $2
            WHERE name = $3
            "#,
            self.public_url,
            self.enabled,
            &self.name as &GameName,
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    pub async fn rename<Conn>(&mut self, new_name: GameName, mut conn: Conn) -> anyhow::Result<()>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        sqlx::query_as!(
            Self,
            r#"
            UPDATE game_servers
            SET name = $1
            WHERE name = $2
            "#,
            &new_name as &GameName,
            &self.name as &GameName,
        )
        .execute(&mut *conn)
        .await?;
        self.name = new_name;
        Ok(())
    }

    pub async fn delete<Conn>(self, mut conn: Conn) -> anyhow::Result<()>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        sqlx::query!(
            "DELETE FROM game_servers WHERE name = $1",
            self.name as GameName
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
