mod game_id;
mod postgres;
mod redis;
mod routes;
mod token;
mod user_id;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let redis_pool = redis::connect().await?;
    let pg_pool = postgres::connect().await?;

    rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::challenge::challenge,
                routes::leaderboard::leaderboard
            ],
        )
        .manage(redis_pool)
        .manage(pg_pool)
        .launch()
        .await?;
    Ok(())
}
