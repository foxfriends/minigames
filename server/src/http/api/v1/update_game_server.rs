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
pub struct UpdateGameServerRequest {
    name: Option<GameName>,
    public_url: Option<String>,
}

#[rocket::patch("/servers/<name>", data = "<body>")]
pub async fn update_game_server(
    db: &State<PgPool>,
    body: Json<UpdateGameServerRequest>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Json<GameServer>> {
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
    server.save(&mut conn).await?;

    conn.commit().await?;
    Ok(Json(server))
}
