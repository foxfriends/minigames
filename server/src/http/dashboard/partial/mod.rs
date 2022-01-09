use super::DashboardContext;

mod construction;
mod header;
mod layout;
mod logo;
mod nav;
mod page;

pub use construction::construction;
use header::header;
pub use layout::layout;
use logo::logo;
use nav::nav;
pub use page::{full_page, page};
