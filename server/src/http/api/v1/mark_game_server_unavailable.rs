use crate::game::{ApiKeys, GameName, GameRegistry};
use crate::http::api_key_header::ApiKeyHeader;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::http::Status;
use rocket::State;

#[rocket::post("/servers/<game>/unavailable")]
pub async fn mark_game_server_unavailable(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    game: GameName,
    api_key: ApiKeyHeader,
) -> Response<Status> {
    let mut conn = db.acquire().await?;
    let api_keys = ApiKeys::load(&game, &mut conn).await?;
    if api_key.value() != &api_keys.secret_key {
        return Err(ResponseError::new(
            Status::Forbidden,
            "IncorrectApiKey".to_owned(),
            "This API key is not valid for this game".to_owned(),
        ));
    }
    registry.set_available(&game, false).await;
    Ok(Status::NoContent)
}
