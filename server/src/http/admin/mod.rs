use rocket::Route;

mod index;
mod partial;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![index::index]
}
