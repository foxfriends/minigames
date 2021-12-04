mod game;
mod guild;
mod postgres;
mod response;
mod routes;
mod token;
mod user;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let pg_pool = postgres::connect().await?;

    rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::create_challenge::create_challenge,
                routes::get_challenge::get_challenge,
                routes::get_challenge::complete_oauth2,
                routes::leaderboard::leaderboard,
            ],
        )
        .manage(pg_pool)
        .launch()
        .await?;
    Ok(())
}
