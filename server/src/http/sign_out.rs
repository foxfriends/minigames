use super::cookies::UserCookie;
use rocket::http::CookieJar;
use rocket::response::Redirect;

#[rocket::get("/sign-out")]
pub fn sign_out(cookie_jar: &CookieJar<'_>) -> Redirect {
    UserCookie::remove_from(cookie_jar);
    Redirect::to("/")
}
