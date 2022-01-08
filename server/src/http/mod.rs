use rocket::fs::FileServer;

use crate::game::GameRegistry;
use crate::postgres::PgPool;

mod authorization;
mod cookies;
mod cors;
mod response;

pub use cors::CorsOrigin;

mod add_to_server;
mod admin;
mod complete_game;
mod create_challenge;
mod get_game;
mod get_public_key;
mod leaderboard;
mod list_games;
mod oauth;
mod play_game;
mod register_game;
mod unregister_game;

pub async fn server(pg_pool: PgPool) -> anyhow::Result<()> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
                add_to_server::add_to_server,
                complete_game::complete_game,
                create_challenge::create_challenge,
                get_game::get_game,
                play_game::play_game,
                play_game::sign_in_then_play_game,
                oauth::complete_oauth2,
                get_public_key::get_public_key,
                leaderboard::leaderboard,
                list_games::list_games,
                register_game::register_game,
                unregister_game::unregister_game,
            ],
        )
        .mount("/dashboard", admin::routes())
        .mount("/static", FileServer::from(crate::env::static_files_dir()))
        .attach(cors::Cors)
        .manage(pg_pool)
        .manage(GameRegistry::default())
        .launch()
        .await?;
    Ok(())
}
