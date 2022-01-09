use crate::discord;
use crate::game::{GameName, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{uri, State};
use serde::Deserialize;

#[derive(Deserialize, FromForm)]
pub struct CreateGameServerRequest {
    name: GameName,
    public_url: String,
}

async fn create_game_server(
    db: &State<PgPool>,
    body: &CreateGameServerRequest,
    user_cookie: UserCookie<'_>,
) -> Response<GameServer> {
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
    Ok(created_server)
}

#[rocket::post("/servers/new", data = "<body>", format = "json")]
pub async fn create_game_server_json(
    db: &State<PgPool>,
    body: Json<CreateGameServerRequest>,
    user_cookie: UserCookie<'_>,
) -> Response<Json<GameServer>> {
    let created_server = create_game_server(db, &*body, user_cookie).await?;
    Ok(Json(created_server))
}

#[rocket::post("/servers/new", data = "<body>", format = "form")]
pub async fn create_game_server_form(
    db: &State<PgPool>,
    body: Form<CreateGameServerRequest>,
    user_cookie: UserCookie<'_>,
) -> Response<Redirect> {
    let server = create_game_server(db, &*body, user_cookie).await?;
    Ok(Redirect::to(uri!(
        "/dashboard",
        crate::http::dashboard::admin::servers::edit::edit(server.name())
    )))
}
