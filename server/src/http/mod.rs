use crate::postgres::PgPool;

mod create_challenge;
mod get_challenge;
mod leaderboard;

pub async fn server(pg_pool: PgPool) -> anyhow::Result<()> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
                create_challenge::create_challenge,
                get_challenge::get_challenge,
                get_challenge::complete_oauth2,
                leaderboard::leaderboard,
            ],
        )
        .manage(pg_pool)
        .launch()
        .await?;
    Ok(())
}
