use crate::asset::Asset;
use crate::discord;
use crate::game::{GameName, GameRegistry, GameServer};
use crate::guild::GuildId;
use crate::http::cookies::UserCookie;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
use rocket::http::{ContentType, Status};
use rocket::response::Redirect;
use rocket::{uri, State};

#[derive(FromForm)]
pub struct UpdateGameServerRequest<'r> {
    name: Option<GameName>,
    public_url: Option<String>,
    guilds: Option<Vec<GuildId>>,
    enabled: bool,
    asset: Option<TempFile<'r>>,
}

fn get_extension(file: &TempFile<'_>) -> Response<String> {
    let content_type = file.content_type().ok_or_else(|| {
        ResponseError::new(
            Status::BadRequest,
            "MissingContentType".to_owned(),
            "Content-Type of the file must be specified".to_owned(),
        )
    })?;
    if content_type != &ContentType::PNG
        && content_type != &ContentType::JPEG
        && content_type != &ContentType::GIF
    {
        return Err(ResponseError::new(
            Status::UnsupportedMediaType,
            "InvalidContentType".to_owned(),
            "Only PNG, GIF, and JPEG images are allowed".to_owned(),
        ));
    }

    let extension = content_type.extension().unwrap().as_str().to_lowercase();
    Ok(extension)
}

#[rocket::post(
    "/admin/servers/<name>",
    data = "<body>",
    format = "multipart/form-data"
)]
pub async fn update_game_server(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    mut body: Form<UpdateGameServerRequest<'_>>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Redirect> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let mut conn = db.begin().await?;
    let mut server = match GameServer::load(&name, &mut conn).await? {
        Some(server) if server.user_id() == user.id => server,
        _ => {
            return Err(ResponseError::new(
                Status::Forbidden,
                "NotYourServer".to_owned(),
                "You cannot modify a game server you do not own".to_owned(),
            ))
        }
    };

    if let Some(name) = &body.name {
        server.rename(name.clone(), &mut conn).await?;
    }
    if let Some(public_url) = &body.public_url {
        server.public_url = public_url.to_owned();
    }
    server.enabled = body.enabled;

    if let Some(file) = &mut body.asset {
        let asset = Asset::create(&get_extension(file)?, &mut conn).await?;
        file.persist_to(asset.path()).await?;
        let previous_asset = server.asset_id.replace(asset.id);
        if let Some(previous_id) = previous_asset {
            Asset::load(previous_id, &mut conn)
                .await?
                .delete(&mut conn)
                .await?;
        }
    }

    server.save(&mut conn).await?;

    if let Some(guilds) = &body.guilds {
        server.set_guilds(guilds, &mut conn).await?;
    }

    conn.commit().await?;

    if let Some(name) = &body.name {
        registry.unregister(name).await;
    }
    registry.register(&server).await;

    Ok(Redirect::to(uri!(
        "/dashboard",
        super::edit::edit(server.name())
    )))
}
