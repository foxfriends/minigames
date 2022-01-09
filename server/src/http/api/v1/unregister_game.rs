use crate::game::{GameName, GameRegistry};
use crate::http::response::Response;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterGameRequest {
    url: String,
}

#[rocket::delete("/games/<game>")]
pub async fn unregister_game(registry: &State<GameRegistry>, game: GameName) -> Response<()> {
    registry.unregister(&game).await;
    Ok(())
}
