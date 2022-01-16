use super::create_game_server;
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{button, field, layout, page, text_input};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;
use rocket::uri;

#[rocket::get("/admin/servers/new")]
pub async fn new(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(user_cookie.value())
        .await?
        .with_path(["Developer", "New Game"])
        .build();
    let markup = layout(
        &ctx,
        page(html! {
            form.flex.flex-col."gap-6" method="POST" action=(uri!("/dashboard", create_game_server::create_game_server())) {
                (field(
                    "Name",
                    "name",
                    text_input("name", "", ""),
                ))
                (field(
                    "Public URL",
                    "public_url",
                    text_input("public_url", "https://my-cool-minigame.com", ""),
                ))
                (button(html! { "Create" }))
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
