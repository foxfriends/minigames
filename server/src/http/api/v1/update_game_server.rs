use crate::discord;
use crate::game::{GameName, GameRegistry, GameServer};
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
pub struct UpdateGameServerRequest {
    name: Option<GameName>,
    public_url: Option<String>,
    enabled: bool,
}

async fn update_game_server(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    body: &UpdateGameServerRequest,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<GameServer> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let mut conn = db.begin().await?;
    let mut server = match GameServer::load(&name, &mut conn).await? {
        Some(server) => server,
        None => {
            return Err(ResponseError::new(
                Status::NotFound,
                "NoSuchServer".to_owned(),
                format!("No server named {} was found", name),
            ));
        }
    };
    if server.user_id() != user.id {
        return Err(ResponseError::new(
            Status::Forbidden,
            "NotYourServer".to_owned(),
            "You cannot modify a game server you do not own".to_owned(),
        ));
    }

    if let Some(name) = &body.name {
        server.rename(name.clone(), &mut conn).await?;
    }
    if let Some(public_url) = &body.public_url {
        server.public_url = public_url.to_owned();
    }
    server.enabled = body.enabled;

    server.save(&mut conn).await?;
    conn.commit().await?;

    if let Some(name) = &body.name {
        registry.unregister(name).await;
    }
    registry.register(&server).await;

    Ok(server)
}

#[rocket::patch("/servers/<name>", data = "<body>", format = "json")]
pub async fn update_game_server_json(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    body: Json<UpdateGameServerRequest>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Json<GameServer>> {
    let updated_server = update_game_server(db, registry, &*body, name, user_cookie).await?;
    Ok(Json(updated_server))
}

#[rocket::patch("/servers/<name>", data = "<body>", format = "form")]
pub async fn update_game_server_form(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    body: Form<UpdateGameServerRequest>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Redirect> {
    let updated_server = update_game_server(db, registry, &*body, name, user_cookie).await?;
    Ok(Redirect::to(uri!(
        "/dashboard",
        crate::http::dashboard::admin::servers::edit::edit(updated_server.name())
    )))
}
