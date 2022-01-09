use rocket::Route;

mod add_to_server;
mod complete_game;
mod create_challenge;
mod get_game;
mod leaderboard;
mod list_games;
mod register_game;
mod unregister_game;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        add_to_server::add_to_server,
        complete_game::complete_game,
        create_challenge::create_challenge,
        get_game::get_game,
        leaderboard::leaderboard,
        list_games::list_games,
        register_game::register_game,
        unregister_game::unregister_game,
    ]
}
