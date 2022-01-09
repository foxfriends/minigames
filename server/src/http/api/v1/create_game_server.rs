use crate::discord;
use crate::game::{GameName, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use crate::postgres::PgPool;
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateGameServerRequest {
    name: GameName,
    url: String,
}

#[rocket::post("/servers/new", data = "<body>")]
pub async fn create_game_server(
    db: &State<PgPool>,
    body: Json<CreateGameServerRequest>,
    user_cookie: UserCookie<'_>,
) -> Response<Json<GameServer>> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let mut conn = db.acquire().await?;
    let created_server = GameServer::create(&body.name, user.id, &body.url, &mut conn).await?;
    Ok(Json(created_server))
}
