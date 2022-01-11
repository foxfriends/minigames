use rocket::fs::FileServer;

use crate::game::GameRegistry;
use crate::postgres::PgPool;

mod api_key_header;
mod cookies;
mod cors;
mod response;
mod user_authorization;

mod api;
mod auth;
mod dashboard;

mod add_to_server;
mod get_public_key;
mod index;
mod play_game;

pub use cors::CorsOrigin;

pub async fn server(pg_pool: PgPool, registry: GameRegistry) -> anyhow::Result<()> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
                index::index,
                add_to_server::add_to_server,
                get_public_key::get_public_key,
                play_game::play_game,
                play_game::sign_in_then_play_game
            ],
        )
        .mount("/api/v1", api::v1::routes())
        .mount("/auth", auth::routes())
        .mount("/dashboard", dashboard::routes())
        .mount("/static", FileServer::from(crate::env::static_files_dir()))
        .attach(cors::Cors)
        .manage(pg_pool)
        .manage(registry)
        .launch()
        .await?;
    Ok(())
}
