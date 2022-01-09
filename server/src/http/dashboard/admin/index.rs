use crate::game::GameServer;
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{empty, h1, layout, link_button, page};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use crate::postgres::PgPool;
use maud::{html, Markup};
use rocket::response::content::Html;
use rocket::{uri, State};

fn game_server_tile(_ctx: &DashboardContext, server: &GameServer) -> Markup {
    html! {
        .w-min.h-min {
            ."w-32"."h-32".rounded-sm.bg-background-floating.text-text-heading {
                (server.name()[0..1])
            }
            .truncate.text.text-heading.center {
                (server.name())
            }
        }
    }
}

#[rocket::get("/admin")]
pub async fn index(db: &State<PgPool>, user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::load(["Server Admin"], user_cookie.value()).await?;
    let mut conn = db.acquire().await?;
    let game_servers = GameServer::list_all(&mut conn).await?;

    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-6" {
                .flex.items-center.justify-between.w-full {
                    (h1(html! { "Installed Games" }))
                    (link_button(uri!("/dashboard", super::servers::new::new()), html! { "New Game" }))
                }
                @if game_servers.is_empty() {
                    (empty(html! {
                        "Looks like there aren't any games installed yet. Why not make one now?"
                    }))
                } @else {
                    .flex.flex-row.flex-wrap {
                        @for server in &game_servers {
                            (game_server_tile(&ctx, server))
                        }
                    }
                }
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
