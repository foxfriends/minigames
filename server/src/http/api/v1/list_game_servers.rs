use crate::discord;
use crate::game::GameServer;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use crate::postgres::PgPool;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct ListGameServersResponse {
    servers: Vec<GameServer>,
}

#[rocket::get("/servers")]
pub async fn list_game_servers(
    db: &State<PgPool>,
    user_cookie: UserCookie<'_>,
) -> Response<Json<ListGameServersResponse>> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let mut conn = db.acquire().await?;
    let servers = GameServer::list_for_user(user.id, &mut conn).await?;
    Ok(Json(ListGameServersResponse { servers }))
}
