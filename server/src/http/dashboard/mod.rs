use rocket::Route;

mod context;

use context::DashboardContext;

pub mod admin;
pub mod index;
pub mod partial;
pub mod sign_in;

pub fn routes() -> impl Into<Vec<Route>> {
    rocket::routes![
        index::index,
        sign_in::sign_in,
        admin::index::index,
        admin::servers::new::new,
        admin::servers::edit::edit,
    ]
}
