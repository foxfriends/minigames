use crate::discord::get_user;
use crate::game::{Game, GameId, GameName};
use crate::http::response::Response;
use crate::postgres::PgPool;
use crate::user::UserId;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePlayer {
    // Participant info
    pub id: UserId,
    pub is_challenger: bool,
    // Discord user info
    pub username: String,
    pub discriminator: String,
    pub avatar: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGameResponse {
    players: Vec<GamePlayer>,
    is_complete: bool,
    winner_id: Option<UserId>,
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
    let participants = game.participants(&mut conn).await?;

    let winner_id = participants
        .iter()
        .find(|participant| participant.score > 0)
        .map(|winner| winner.id);

    let mut players = Vec::with_capacity(participants.len());
    for participant in participants {
        let user = get_user(participant.id).await?;
        players.push(GamePlayer {
            id: participant.id,
            is_challenger: participant.is_challenger,
            username: user.username,
            discriminator: user.discriminator,
            avatar: user.avatar,
        })
    }

    Ok(Json(GetGameResponse {
        players,
        is_complete: game.complete,
        winner_id,
    }))
}
