use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Challenge;

#[rocket::post("/challenge")]
pub fn challenge() -> Result<Json<Challenge>, Status> {
    Ok(Json(Challenge))
}
