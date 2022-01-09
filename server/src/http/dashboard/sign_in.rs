use crate::http::auth::sign_in_with_discord;
use crate::http::response::Response;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::route::Route;
use std::path::PathBuf;

#[rocket::get("/<path..>")]
pub async fn sign_in(route: &Route, path: PathBuf, cookies: &CookieJar<'_>) -> Response<Redirect> {
    // Is this really the only way to get the path of the current request? It seems pretty unreliable.
    sign_in_with_discord(format!("{}/{}", route.uri.base, path.display()), cookies).await
}
