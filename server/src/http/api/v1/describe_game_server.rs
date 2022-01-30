use crate::env::public_http_url;
use crate::game::{GameName, GameServer};
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameServerDescription {
    name: GameName,
    image_url: Option<String>,
}

#[rocket::get("/servers/<game>")]
pub async fn describe_game_server(
    db: &State<PgPool>,
    game: GameName,
) -> Response<Json<GameServerDescription>> {
    let mut conn = db.acquire().await?;
    let server = match GameServer::load(&game, &mut conn).await? {
        Some(server) => server,
        None => {
            return Err(ResponseError::new(
                Status::NotFound,
                "GameServerNotFound".to_owned(),
                "No server by that name is known".to_owned(),
            ))
        }
    };
    Ok(Json(GameServerDescription {
        name: game,
        image_url: server
            .asset(&mut conn)
            .await?
            .map(|asset| format!("{}{}", public_http_url(), asset.url())),
    }))
}
