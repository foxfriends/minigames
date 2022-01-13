use crate::game::{GameName, GameRegistry, GameServer};
use crate::guild::GuildId;
use crate::http::response::Response;
use crate::postgres::PgPool;
use rocket::serde::json::Json;
use rocket::State;

#[rocket::get("/games?<guild_id>")]
pub async fn list_games(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    guild_id: Option<GuildId>,
) -> Response<Json<Vec<GameName>>> {
    let mut conn = db.acquire().await?;
    let servers = registry.list_available().await;

    if let Some(guild_id) = guild_id {
        let mut servers_to_return = Vec::with_capacity(servers.len());
        for game_name in servers {
            let add = GameServer::load(&game_name, &mut conn)
                .await?
                .unwrap()
                .is_in_guild(guild_id, &mut conn)
                .await
                .unwrap_or(false);
            if add {
                servers_to_return.push(game_name);
            }
        }
        Ok(Json(servers_to_return))
    } else {
        Ok(Json(servers))
    }
}
