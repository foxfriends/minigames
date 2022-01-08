use sqlx::postgres::PgPoolOptions;

pub use sqlx::postgres::PgPool;

pub async fn connect() -> anyhow::Result<PgPool> {
    let postgres_url = crate::env::database_url();
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_url)
        .await?)
}
