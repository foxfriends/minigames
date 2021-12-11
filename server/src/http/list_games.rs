use super::response::Response;
use crate::game::{GameName, GameRegistry};
use rocket::serde::json::Json;
use rocket::State;

#[rocket::get("/games")]
pub async fn list_games(registry: &State<GameRegistry>) -> Response<Json<Vec<GameName>>> {
    Ok(Json(registry.list_all().await))
}
