use rocket::route::Route;

pub mod oauth;
pub mod sign_out;

pub use oauth::sign_in_with_discord;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![oauth::complete_oauth2, sign_out::sign_out,]
}
