use super::partial::{construction, full_page, layout};
use super::DashboardContext;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;

#[rocket::get("/")]
pub async fn index(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let ctx = DashboardContext::builder(["Dashboard"])
        .load_user(user_cookie.value())
        .await?
        .build();
    let markup = layout(
        &ctx,
        full_page(html! {
            .flex.items-center.justify-center.w-full.h-full {
                (construction("The dashboard is still under construction. Hopefully we'll have something to show soon!"))
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
