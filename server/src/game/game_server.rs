use super::{ApiKeys, GameName};
use crate::user::UserId;
use sqlx::postgres::Postgres;
use sqlx::Executor;

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
        name: GameName,
        user_id: UserId,
        public_url: String,
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
            &name as &GameName,
            user_id as UserId,
            public_url,
        )
        .fetch_one(&mut *conn)
        .await?;
        ApiKeys::generate(name, &mut *conn).await?;
        Ok(game_server)
    }
}
