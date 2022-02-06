use crate::game::GameName;
use crate::guild::GuildId;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use crate::user::UserId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScore {
    score: Vec<i64>,
    total_games: u64,
}

#[derive(Default)]
struct Scores {
    p1score: Option<i64>,
    p2score: Option<i64>,
    total_games: Option<i64>,
}

#[rocket::get("/score?<guild_id>&<users>&<game>")]
pub async fn get_score(
    db: &State<PgPool>,
    game: Option<GameName>,
    guild_id: GuildId,
    users: Vec<UserId>,
) -> Response<Json<GetScore>> {
    let mut conn = db.acquire().await?;
    if users.len() != 2 {
        return Err(ResponseError::new(
            Status::BadRequest,
            "InvalidPlayers".to_owned(),
            "Getting score requires exactly 2 players".to_owned(),
        ));
    }
    let scores = match game {
        Some(game) =>
        sqlx::query_as!(Scores, r#"
            SELECT SUM(p1.score) AS p1score, SUM(p2.score) AS p2score, count(p1.game_id) AS total_games
                FROM game_participants p1
                INNER JOIN game_participants p2 ON p1.game_id = p2.game_id
                INNER JOIN games ON games.id = p1.game_id
                WHERE games.complete = true
                AND games.game = $1
                AND games.guild_id = $2
                AND p1.user_id = $3
                AND p2.user_id = $4
        "#, game as GameName, guild_id as GuildId, users[0] as UserId, users[1] as UserId)
            .fetch_optional(&mut conn)
            .await?
            .unwrap_or_default(),
        None =>
        sqlx::query_as!(Scores, r#"
            SELECT SUM(p1.score) AS p1score, SUM(p2.score) AS p2score, count(p1.game_id) AS total_games
                FROM game_participants p1
                INNER JOIN game_participants p2 ON p1.game_id = p2.game_id
                INNER JOIN games ON games.id = p1.game_id
                WHERE games.complete = true
                AND games.guild_id = $1
                AND p1.user_id = $2
                AND p2.user_id = $3
        "#, guild_id as GuildId, users[0] as UserId, users[1] as UserId)
            .fetch_optional(&mut conn)
            .await?
            .unwrap_or_default(),
    };

    Ok(Json(GetScore {
        score: vec![scores.p1score.unwrap_or(0), scores.p2score.unwrap_or(0)],
        total_games: scores.total_games.unwrap_or(0) as u64,
    }))
}
