use super::{ApiKeys, GameName};
use crate::user::UserId;
use serde::{Deserialize, Serialize};
use sqlx::postgres::Postgres;
use sqlx::Executor;

#[derive(Serialize, Deserialize)]
pub struct GameServer {
    name: GameName,
    user_id: UserId,
    pub public_url: String,
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
                    public_url
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

    pub async fn list_for_user<Conn>(user_id: UserId, mut conn: Conn) -> anyhow::Result<Vec<Self>>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let servers = sqlx::query_as!(
            Self,
            r#"
            SELECT name as "name: _", user_id as "user_id: _", public_url
            FROM game_servers
            WHERE user_id = $1
            "#,
            user_id as UserId,
        )
        .fetch_all(&mut *conn)
        .await?;
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
            SELECT name as "name: _", user_id as "user_id: _", public_url
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
            SET public_url = $1
            WHERE name = $2
            "#,
            self.public_url,
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
