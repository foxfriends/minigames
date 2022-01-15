mod asset;
mod discord;
mod env;
mod game;
mod guild;
mod http;
mod jobs;
mod key;
mod postgres;
mod token;
mod user;
mod ws;

use crate::game::GameRegistry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let pg_pool = postgres::connect().await?;
    let registry = {
        let mut conn = pg_pool.acquire().await?;
        GameRegistry::initialize(&mut conn).await?
    };

    let http_server = http::server(pg_pool.clone(), registry.clone());
    let ws_server = ws::server(pg_pool.clone());
    let jobs = jobs::jobs(pg_pool, registry);
    futures::pin_mut!(http_server, ws_server);
    let servers = futures::future::select(http_server, ws_server);
    futures::pin_mut!(servers, jobs);
    futures::future::select(jobs, servers).await;
    Ok(())
}
