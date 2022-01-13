use crate::game::{GameName, GameServer};
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

#[rocket::get("/admin")]
pub async fn index(db: &State<PgPool>, user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(user_cookie.value())
        .await?
        .with_path(["Developer"])
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
                (page_heading(
                    "Your Games",
                    Some(link_button(uri!("/dashboard", super::servers::new::new()), html! { "New Game" }))
                ))
                @if game_servers.is_empty() {
                    (empty(html! {
                        "Looks like you haven't added any games yet. Got a suggestion?"
                    }))
                } @else {
                    .flex.flex-row.flex-wrap."gap-4" {
                        @for server in &game_servers {
                            (game_server_tile(&ctx, server, Some(uri!("/dashboard", super::super::admin::servers::edit::edit(server.name())))).await)
                        }
                    }
                }
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
