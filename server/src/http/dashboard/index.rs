use super::partial::{empty, guild_row, layout, page, page_heading};
use super::DashboardContext;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;

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
                (page_heading("Choose a Server", None))
                @if guilds.is_empty() {
                    (empty(html! {
                        "You are not a member of any server that has the Discord Party bot installed."
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
