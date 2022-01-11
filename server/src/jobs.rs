use crate::game::{ApiKeys, GameName, GameRegistry};
use crate::postgres::PgPool;
use crate::token::HealthCheckClaims;
use reqwest::Client;
use rocket::{info, warn};
use tokio::time::{sleep, Duration};

async fn health_check(game: &GameName, url: String, db: &PgPool) -> anyhow::Result<()> {
    let token = HealthCheckClaims::new(game.clone()).sign()?;
    let request = Client::new()
        .post(format!("{}/health", url))
        .header("X-Minigames-Server", token);
    let response = request.send().await?;
    let mut conn = db.acquire().await?;
    let api_keys = ApiKeys::load(game, &mut conn).await?;
    let api_key = match response.headers().get("X-Api-Key") {
        Some(header) => header.to_str()?.parse()?,
        None => anyhow::bail!("The response did not include an API key"),
    };
    anyhow::ensure!(
        api_keys.secret_key == api_key,
        "The returned API key is not valid"
    );
    Ok(())
}

pub async fn jobs(db: PgPool, registry: GameRegistry) -> anyhow::Result<()> {
    loop {
        for (game, url) in registry.list_enabled().await {
            match health_check(&game, url, &db).await {
                Err(error) => {
                    warn!("Health check failed for {}: {}", game, error);
                    registry.set_available(&game, false).await;
                }
                Ok(..) => {
                    info!("{} is healthy", game);
                    registry.set_available(&game, true).await;
                }
            }
        }
        // We really don't need to check this very often.
        sleep(Duration::from_secs(60 * 20)).await;
    }
}
