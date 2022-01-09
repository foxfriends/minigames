use crate::discord;
use crate::game::{GameName, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateGameServerRequest {
    name: GameName,
    public_url: String,
}

#[rocket::post("/servers/new", data = "<body>")]
pub async fn create_game_server(
    db: &State<PgPool>,
    body: Json<CreateGameServerRequest>,
    user_cookie: UserCookie<'_>,
) -> Response<Json<GameServer>> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let mut conn = db.acquire().await?;
    if body.name.is_empty() {
        return Err(ResponseError::new(
            Status::BadRequest,
            "BadRequest".to_owned(),
            "Server name must not be empty".to_owned(),
        ));
    }
    if body.public_url.is_empty() {
        return Err(ResponseError::new(
            Status::BadRequest,
            "BadRequest".to_owned(),
            "Server public_url must not be empty".to_owned(),
        ));
    }
    let created_server =
        GameServer::create(&body.name, user.id, &body.public_url, &mut conn).await?;
    Ok(Json(created_server))
}
