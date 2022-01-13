use crate::game::{GameRegistry, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{empty, game_server_tile, h1, layout, link_button, page};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use crate::postgres::PgPool;
use maud::html;
use rocket::response::content::Html;
use rocket::{uri, State};

#[rocket::get("/library")]
pub async fn library(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    user_cookie: UserCookie<'_>,
) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(["Library"])
        .with_registry((*registry).clone())
        .load_user(user_cookie.value())
        .await?
        .build();
    let mut conn = db.acquire().await?;
    let game_servers = GameServer::list_all(&mut conn).await?;

    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-6" {
                .flex.items-center.justify-between.w-full {
                    (h1(html! { "Installed Games" }))
                    (link_button(uri!("/dashboard", super::admin::index::index()), html! { "Manage" }))
                }
                @if game_servers.is_empty() {
                    (empty(html! {
                        "Looks like there aren't any games installed yet. If this is your server, why not set some up?"
                    }))
                } @else {
                    .flex.flex-row.flex-wrap."gap-4" {
                        @for server in &game_servers {
                            (game_server_tile(&ctx, server).await)
                        }
                    }
                }
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
