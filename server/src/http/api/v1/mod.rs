use rocket::Route;

pub mod complete_game;
pub mod create_challenge;
pub mod get_game;
pub mod leaderboard;
pub mod list_games;

pub mod create_game_server;
pub mod delete_game_server;
pub mod mark_game_server_available;
pub mod mark_game_server_unavailable;
pub mod update_game_server;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        // Game APIs
        complete_game::complete_game,
        get_game::get_game,
        // Bot APIs
        create_challenge::create_challenge,
        leaderboard::leaderboard,
        list_games::list_games,
        // Admin APIs
        create_game_server::create_game_server_json,
        create_game_server::create_game_server_form,
        update_game_server::update_game_server_json,
        update_game_server::update_game_server_form,
        delete_game_server::delete_game_server,
        // Game server APIs
        mark_game_server_available::mark_game_server_available,
        mark_game_server_unavailable::mark_game_server_unavailable,
    ]
}
