use crate::game::{Game, GameId};
use crate::http::response::{Response, ResponseError};
use crate::http::user_authorization::UserAuthorization;
use crate::postgres::PgPool;
use crate::user::UserId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteGameRequest {
    game_id: GameId,
    winner_id: Option<UserId>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteGameResponse {
    winner_id: Option<UserId>,
}

#[rocket::post("/complete", data = "<body>")]
pub async fn complete_game(
    db: &State<PgPool>,
    auth: UserAuthorization,
    body: Json<CompleteGameRequest>,
) -> Response<Json<Option<CompleteGameResponse>>> {
    let mut conn = db.begin().await?;
    let game = Game::load(body.game_id, &mut conn).await?;
    if let Some(winner_id) = body.winner_id {
        if !game.is_participant(winner_id, &mut conn).await? {
            return Err(ResponseError::new(
                Status::BadRequest,
                "InvalidWinner".to_owned(),
                "The winner must be a participant in the game".to_owned(),
            ));
        }
    }
    game.vote_result(auth.user_id(), body.winner_id, &mut conn)
        .await?;
    let winner_id = game.check_winner(&mut conn).await?;
    conn.commit().await?;
    Ok(Json(
        winner_id.map(|winner_id| CompleteGameResponse { winner_id }),
    ))
}
