use crate::game::{Game, GameId, GameName, GameRegistry};
use crate::guild::{Guild, GuildId};
use crate::postgres::PgPool;
use crate::response::{Response, ResponseError};
use crate::user::{User, UserId};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallengeResponse {
    game_id: GameId,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallengeRequest {
    guild_id: GuildId,
    challenger_id: UserId,
    challenged_id: UserId,
    game: GameName,
}

#[rocket::post("/challenge", data = "<body>")]
pub async fn create_challenge(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    body: Json<CreateChallengeRequest>,
) -> Response<Json<CreateChallengeResponse>> {
    if registry.locate(&body.game).await.is_none() {
        return Err(ResponseError::new(
            Status::NotFound,
            "GameNotRegistered".to_owned(),
            format!("Game named {} is unknown", body.game),
        ));
    }

    let mut conn = db.begin().await?;
    Guild::upsert(body.guild_id, &mut conn).await?;
    User::upsert(body.challenger_id, &mut conn).await?;
    User::upsert(body.challenged_id, &mut conn).await?;
    let game = Game::create(body.guild_id, body.game.clone(), &mut conn).await?;
    conn.commit().await?;
    Ok(Json(CreateChallengeResponse { game_id: game.id }))
}
