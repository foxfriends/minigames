use rocket::route::Route;

mod oauth;
mod sign_out;

pub use oauth::sign_in_with_discord;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![oauth::complete_oauth2, sign_out::sign_out,]
}
