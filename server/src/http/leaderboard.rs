use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Leaderboard;

#[rocket::post("/leaderboard")]
pub fn leaderboard() -> Result<Json<Leaderboard>, Status> {
    Ok(Json(Leaderboard))
}
