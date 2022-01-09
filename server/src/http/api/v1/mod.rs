use rocket::Route;

mod complete_game;
mod create_challenge;
mod get_game;
mod leaderboard;
mod list_games;
mod register_game;
mod unregister_game;

mod create_game_server;
mod delete_game_server;
mod list_game_servers;
mod update_game_server;

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
        create_game_server::create_game_server,
        update_game_server::update_game_server,
        delete_game_server::delete_game_server,
        list_game_servers::list_game_servers,
        // Legacy APIs
        register_game::register_game,
        unregister_game::unregister_game,
    ]
}
