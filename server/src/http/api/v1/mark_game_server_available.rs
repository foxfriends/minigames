use crate::game::{GameName, GameRegistry};
use crate::http::response::Response;
use rocket::http::Status;
use rocket::State;

#[rocket::post("/servers/<game>/available")]
pub async fn mark_game_server_available(
    registry: &State<GameRegistry>,
    game: GameName,
) -> Response<Status> {
    registry.set_available(&game, true).await;
    Ok(Status::NoContent)
}
