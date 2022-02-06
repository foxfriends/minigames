use rocket::Route;

pub mod complete_game;
pub mod create_challenge;
pub mod get_game;
pub mod get_score;
pub mod list_games;

pub mod describe_game_server;
pub mod mark_game_server_available;
pub mod mark_game_server_unavailable;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        // Game APIs
        complete_game::complete_game,
        get_game::get_game,
        // Bot APIs
        describe_game_server::describe_game_server,
        create_challenge::create_challenge,
        get_score::get_score,
        list_games::list_games,
        // Game server APIs
        mark_game_server_available::mark_game_server_available,
        mark_game_server_unavailable::mark_game_server_unavailable,
    ]
}
