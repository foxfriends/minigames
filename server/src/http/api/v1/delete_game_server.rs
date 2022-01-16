use crate::discord;
use crate::game::{GameName, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::{uri, State};

#[rocket::delete("/servers/<name>")]
pub async fn delete_game_server(
    db: &State<PgPool>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Redirect> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let mut conn = db.begin().await?;
    let server = match GameServer::load(&name, &mut conn).await? {
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
            "You cannot delete a game server you do not own".to_owned(),
        ));
    }
    server.delete(&mut conn).await?;
    conn.commit().await?;
    Ok(Redirect::to(uri!(
        "/dashboard",
        crate::http::dashboard::admin::index::index()
    )))
}
