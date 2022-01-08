use super::partial::{layout, page};
use super::DashboardContext;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;

#[rocket::get("/")]
pub async fn index(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::load("Dashboard", user_cookie.value()).await?;
    let markup = layout(
        &ctx,
        page(html! {
            "Hello world"
        }),
    );
    Ok(Html(markup.into_string()))
}
