use crate::game::{GameName, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{empty, h1, layout, link_button, page};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use crate::postgres::PgPool;
use maud::{html, Markup};
use rocket::response::content::Html;
use rocket::{uri, State};

fn game_server_tile(ctx: &DashboardContext, server: &GameServer) -> Markup {
    let content = html! {
        .w-min.h-min."p-2".bg-background-secondary.rounded-md.flex.flex-col."gap-2" {
            ."w-28"."h-28".rounded-md.bg-background-floating.text-text-heading.flex.items-center.justify-center.font-medium.text-lg {
                (server.name()[0..1])
            }
            .truncate.text.text-text-heading.text-center."min-w-0" {
                (server.name())
            }
        }
    };
    if server.user_id() == ctx.user.id {
        html! {
            a."hover:shadow-xl"."hover:-translate-y-2".transition href=(uri!("/dashboard", super::servers::edit::edit(server.name()))) {
                (content)
            }
        }
    } else {
        content
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
                    .flex.flex-row.flex-wrap."gap-4" {
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
