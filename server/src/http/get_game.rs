use super::response::Response;
use crate::game::{Game, GameId, GameName, GameParticipant};
use crate::postgres::PgPool;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGameResponse {
    players: Vec<GameParticipant>,
}

#[rocket::get("/games/<_game>/<id>")]
pub async fn get_game(
    db: &State<PgPool>,
    _game: GameName,
    id: GameId,
) -> Response<Json<GetGameResponse>> {
    // Funny we don't actually use the game name part of this request, but it's there to make things
    // look nice. Maybe it becomes useful one day... but I suspect not.
    let mut conn = db.acquire().await?;
    let game = Game::load(id, &mut conn).await?;
    let players = game.participants(&mut conn).await?;

    Ok(Json(GetGameResponse { players }))
}