use crate::game::{GameName, GameServer};
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{empty, game_server_tile, h1, layout, link_button, page};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use crate::postgres::PgPool;
use maud::html;
use rocket::response::content::Html;
use rocket::{uri, State};

#[rocket::get("/admin")]
pub async fn index(db: &State<PgPool>, user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(["Server Admin"])
        .load_user(user_cookie.value())
        .await?
        .build();
    let mut conn = db.acquire().await?;
    let game_servers: Vec<_> = GameServer::list_all(&mut conn)
        .await?
        .into_iter()
        .filter(|server| server.user_id() == ctx.user().id)
        .collect();

    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-6" {
                .flex.items-center.justify-between.w-full {
                    (h1(html! { "Your Games" }))
                    (link_button(uri!("/dashboard", super::servers::new::new()), html! { "New Game" }))
                }
                @if game_servers.is_empty() {
                    (empty(html! {
                        "Looks like you haven't installed a game yet yourself. Got any suggestions?"
                    }))
                } @else {
                    .flex.flex-row.flex-wrap."gap-4" {
                        @for server in &game_servers {
                            a."hover:shadow-xl"."hover:-translate-y-2".transition href=(uri!("/dashboard", super::super::admin::servers::edit::edit(server.name()))) {
                                (game_server_tile(&ctx, server).await)
                            }
                        }
                    }
                }
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
