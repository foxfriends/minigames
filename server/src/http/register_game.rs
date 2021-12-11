use super::response::Response;
use crate::game::{GameName, GameRegistry};
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterGameRequest {
    url: String,
}

#[rocket::post("/games/<game>", data = "<body>")]
pub async fn register_game(
    registry: &State<GameRegistry>,
    game: GameName,
    body: Json<RegisterGameRequest>,
) -> Response<()> {
    registry.register(game, body.url.parse()?).await;
    Ok(())
}
