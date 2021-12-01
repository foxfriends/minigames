use crate::game::Game;
use crate::guild::{Guild, GuildId};
use crate::postgres::PgPool;
use crate::response::Response;
use crate::token::{Claims, Token};
use crate::user::{User, UserId};
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Challenge {
    token: Token,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChallenge {
    guild_id: GuildId,
    challenger_id: UserId,
    challenged_id: UserId,
    game: String,
}

#[rocket::post("/challenge", data = "<body>")]
pub async fn create_challenge(
    db: &State<PgPool>,
    body: Json<CreateChallenge>,
) -> Response<Json<Challenge>> {
    let mut conn = db.begin().await?;
    Guild::upsert(body.guild_id, &mut conn).await?;
    User::upsert(body.challenger_id, &mut conn).await?;
    User::upsert(body.challenged_id, &mut conn).await?;
    let game = Game::create(body.guild_id, &body.game, &mut conn).await?;
    conn.commit().await?;
    let claims = Claims::new_challenge(game.guild_id, game.id);
    let token = claims.sign()?;
    Ok(Json(Challenge { token }))
}
