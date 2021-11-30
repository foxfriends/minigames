use sqlx::postgres::PgPoolOptions;
use std::env;

pub use sqlx::postgres::PgPool;

pub async fn connect() -> anyhow::Result<PgPool> {
    let postgres_url = env::var("DATABASE_URL")?;
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await?)
}
