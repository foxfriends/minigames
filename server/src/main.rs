mod cookies;
mod game;
mod guild;
mod http;
mod postgres;
mod response;
mod token;
mod user;
mod ws;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let pg_pool = postgres::connect().await?;
    let http_server = http::server(pg_pool.clone());
    let ws_server = ws::server(pg_pool);
    futures::pin_mut!(http_server, ws_server);
    futures::future::select(http_server, ws_server).await;
    Ok(())
}
