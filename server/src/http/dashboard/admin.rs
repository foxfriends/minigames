use super::partial::{construction, full_page, layout};
use super::DashboardContext;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;

#[rocket::get("/admin")]
pub async fn admin(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::load("Server Admin", user_cookie.value()).await?;
    let markup = layout(
        &ctx,
        full_page(html! {
            .flex.items-center.justify-center.w-full.h-full {
                (construction("Tools for server administrators are still under construction. Sorry for the inconvenience."))
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
