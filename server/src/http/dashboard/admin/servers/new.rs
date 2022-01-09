use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{layout, page};
use crate::http::dashboard::DashboardContext;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;

#[rocket::get("/admin/servers/new")]
pub async fn new(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::load(["Server Admin", "New Game"], user_cookie.value()).await?;
    let markup = layout(&ctx, page(html! { "Hello World" }));
    Ok(Html(markup.into_string()))
}
