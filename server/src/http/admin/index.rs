use super::partial::layout;
use crate::discord;
use crate::http::cookies::UserCookie;
use crate::http::response::Response;
use maud::html;
use rocket::response::content::Html;

#[rocket::get("/")]
pub async fn index(user_cookie: UserCookie<'_>) -> Response<Html<String>> {
    let user = discord::get_current_user(user_cookie.value()).await?;
    let markup = layout(html! {
        "Hello, " (user.username)
    });
    Ok(Html(markup.into_string()))
}
