use crate::game::{GameName, GameRegistry};
use crate::http::response::Response;
use rocket::http::Status;
use rocket::State;

#[rocket::post("/servers/<game>/unavailable")]
pub async fn mark_game_server_unavailable(
    registry: &State<GameRegistry>,
    game: GameName,
) -> Response<Status> {
    registry.set_available(&game, false).await;
    Ok(Status::NoContent)
}
