use crate::env::assets_dir;
use sqlx::postgres::Postgres;
use sqlx::Executor;
use std::path::PathBuf;
use tokio::fs::remove_file;
use uuid::Uuid;

pub struct Asset {
    pub id: Uuid,
    pub ext: String,
}

impl Asset {
    pub fn path(&self) -> PathBuf {
        assets_dir().join(format!("{}.{}", self.id, self.ext))
    }

    pub fn url(&self) -> String {
        format!("/asset/{}.{}", self.id, self.ext)
    }

    pub async fn create<Conn>(ext: &str, mut conn: Conn) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let asset = sqlx::query_as!(
            Self,
            "INSERT INTO assets (ext) VALUES ($1) RETURNING *",
            ext
        )
        .fetch_one(&mut *conn)
        .await?;
        Ok(asset)
    }

    pub async fn load<Conn>(id: Uuid, mut conn: Conn) -> anyhow::Result<Self>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        let asset = sqlx::query_as!(Self, "SELECT * FROM assets WHERE id = $1", id)
            .fetch_one(&mut *conn)
            .await?;
        Ok(asset)
    }

    pub async fn delete<Conn>(self, mut conn: Conn) -> anyhow::Result<()>
    where
        Conn: std::ops::DerefMut,
        for<'t> &'t mut Conn::Target: Executor<'t, Database = Postgres>,
    {
        remove_file(self.path()).await?;
        sqlx::query!("DELETE FROM assets WHERE id = $1", self.id)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }
}
