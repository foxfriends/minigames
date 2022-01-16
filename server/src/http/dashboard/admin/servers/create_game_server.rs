use crate::discord;
use crate::game::{GameName, GameRegistry, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{uri, State};

#[derive(FromForm)]
pub struct CreateGameServerRequest {
    name: GameName,
    public_url: String,
}

#[rocket::post("/admin/servers/new", data = "<body>", format = "form")]
pub async fn create_game_server(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    body: Form<CreateGameServerRequest>,
    user_cookie: UserCookie<'_>,
) -> Response<Redirect> {
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
    registry.register(&created_server).await;
    Ok(Redirect::to(uri!(
        "/dashboard",
        super::edit::edit(created_server.name())
    )))
}
