use std::env;
use redis::Client;

pub async fn connect() -> anyhow::Result<Client> {
    let redis_url = env::var("REDIS_URL")?;
    Ok(Client::open(redis_url)?)
}
