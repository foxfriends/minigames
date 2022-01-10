use super::DashboardContext;

mod button;
mod construction;
mod empty;
mod header;
mod heading;
mod input;
mod layout;
mod logo;
mod nav;
mod page;
mod text;

pub use button::{button, link_button};
pub use construction::construction;
pub use empty::empty;
use header::header;
pub use heading::{h1, h3};
pub use input::{field, info_field, switch, text_input};
pub use layout::layout;
use logo::logo;
use nav::nav;
pub use page::{full_page, page};
pub use text::tt;
