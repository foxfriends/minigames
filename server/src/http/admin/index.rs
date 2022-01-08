use super::partial::layout;
use maud::html;
use rocket::response::content::Html;

#[rocket::get("/")]
pub fn index() -> Html<String> {
    let markup = layout(html! {
        "Hello world"
    });
    Html(markup.into_string())
}
