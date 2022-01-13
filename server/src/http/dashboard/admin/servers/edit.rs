use crate::game::{ApiKeys, GameName, GameServer};
use crate::http::api::v1::update_game_server;
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{
    button, field, info_field, layout, page, switch, text_input, tt,
};
use crate::http::dashboard::DashboardContext;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use maud::html;
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::{uri, State};

#[rocket::get("/admin/servers/<name>")]
pub async fn edit(
    db: &State<PgPool>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Html<String>> {
    let mut conn = db.acquire().await?;
    let ctx = DashboardContext::builder(["Server Admin", &*name])
        .load_user(user_cookie.value())
        .await?
        .build();
    let server = match GameServer::load(&name, &mut conn).await? {
        Some(server) if ctx.user().id == server.user_id() => server,
        Some(..) => {
            return Err(ResponseError::new_empty(Status::Forbidden));
        }
        None => {
            return Err(ResponseError::new_empty(Status::NotFound));
        }
    };
    let api_keys = ApiKeys::load(server.name(), &mut conn).await?;
    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-4" {
                form.flex.flex-col."gap-6" method="POST" action=(uri!("/api/v1", update_game_server::update_game_server_form(server.name()))) {
                    input type="hidden" name="_method" value="PATCH";
                    (field(
                        "Name",
                        "name",
                        text_input("name", "", server.name()),
                    ))
                    (field(
                        "Public URL",
                        "public_url",
                        text_input("public_url", "https://my-cool-minigame.com", &server.public_url),
                    ))
                    (field(
                        "Enabled",
                        "enabled",
                        html! {
                            .flex.items-center.justify-between {
                                "Servers that are not enabled will not be made available for games"
                                (switch("enabled", server.enabled))
                            }
                        }
                    ))
                    (button(html! { "Save" }))
                }
                (info_field(
                    "Public Key",
                    tt(api_keys.public_key),
                ))
                (info_field(
                    "Secret Key",
                    tt(api_keys.secret_key),
                ))
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
