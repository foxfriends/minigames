use rocket::Route;

mod index;
mod partial;
mod sign_in;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![index::index, sign_in::sign_in]
}
