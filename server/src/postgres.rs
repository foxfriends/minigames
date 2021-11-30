use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

type PgPool = Pool<Postgres>;

pub async fn connect() -> anyhow::Result<PgPool> {
    let postgres_url = env::var("DATABASE_URL")?;
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await?)
}
