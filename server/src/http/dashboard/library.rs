use crate::game::{GameRegistry, GameServer};
use crate::guild::GuildId;
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{
    empty, game_server_tile, layout, link_button, page, page_heading,
};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use crate::postgres::PgPool;
use maud::html;
use rocket::response::content::Html;
use rocket::{uri, State};

#[rocket::get("/<guild_id>/library")]
pub async fn library(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    guild_id: GuildId,
    user_cookie: UserCookie<'_>,
) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(user_cookie.value())
        .await?
        .with_path(["Dashboard"])
        .with_guild(guild_id)
        .await?
        .with_registry((*registry).clone())
        .build();
    let mut conn = db.acquire().await?;
    let game_servers = GameServer::list_all_for_guild(&guild_id, &mut conn).await?;

    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-6" {
                (page_heading(
                    "Installed Games",
                    Some(link_button(uri!("/dashboard", super::admin::index::index()), html! { "Manage" }))
                ))
                @if game_servers.is_empty() {
                    (empty(html! {
                        "Looks like there aren't any games installed yet. If this is your server, why not set some up?"
                    }))
                } @else {
                    .flex.flex-row.flex-wrap."gap-4" {
                        @for server in &game_servers {
                            (game_server_tile(&ctx, server, None).await)
                        }
                    }
                }
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
