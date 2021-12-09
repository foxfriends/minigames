use crate::game::GameRegistry;
use crate::postgres::PgPool;

mod create_challenge;
mod get_challenge;
mod leaderboard;
mod list_games;
mod register_game;
mod unregister_game;

pub async fn server(pg_pool: PgPool) -> anyhow::Result<()> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
                create_challenge::create_challenge,
                get_challenge::get_challenge,
                get_challenge::complete_oauth2,
                leaderboard::leaderboard,
                register_game::register_game,
                unregister_game::unregister_game,
                list_games::list_games,
            ],
        )
        .manage(pg_pool)
        .manage(GameRegistry::default())
        .launch()
        .await?;
    Ok(())
}
