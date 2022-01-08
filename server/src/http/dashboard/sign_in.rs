use crate::http::oauth::sign_in_with_discord;
use crate::http::response::Response;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use std::path::PathBuf;

#[rocket::get("/<path..>")]
pub async fn sign_in(path: PathBuf, cookies: &CookieJar<'_>) -> Response<Redirect> {
    sign_in_with_discord(path.display().to_string(), cookies).await
}
