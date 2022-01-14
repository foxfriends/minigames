use super::partial::{empty, guild_row, layout, link_button, page, page_heading};
use super::DashboardContext;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;
use rocket::uri;

#[rocket::get("/")]
pub async fn index(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(user_cookie.value())
        .await?
        .with_path(["Dashboard"])
        .build();
    let guilds = ctx.load_guilds().await?;
    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-4" {
                (page_heading("Choose a Server", Some(html! {
                    (link_button(uri!(crate::http::add_to_server::add_to_server()), html! { "Install" }))
                })))
                @if guilds.is_empty() {
                    (empty(html! {
                        "You are not a member of any server that has the Discord Party bot installed. Why not install it on one of your own?"
                    }))
                } @else {
                    "Only servers you and the Discord Party bot are both members of are shown."
                    .grid."gap-4"."grid-cols-3" {
                        @for guild in &guilds {
                            (guild_row(guild))
                        }
                    }
                }
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
