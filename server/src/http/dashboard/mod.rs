use rocket::{Catcher, Route};

mod context;

use context::DashboardContext;

pub mod admin;
pub mod index;
pub mod library;
pub mod partial;
pub mod sign_in;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        library::library,
        index::index,
        admin::index::index,
        admin::servers::new::new,
        admin::servers::edit::edit,
        admin::servers::create_game_server::create_game_server,
        admin::servers::update_game_server::update_game_server,
        admin::servers::delete_game_server::delete_game_server,
    ]
}

pub fn catchers() -> impl Into<Vec<Catcher>> {
    rocket::catchers![sign_in::sign_in]
}
