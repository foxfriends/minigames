use maud::{html, Markup};
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::Request;

fn link(href: &str, text: &str) -> Markup {
    html! { a."text-2xl"."hover:text-text-heading" href=(href) { (text) } }
}

#[rocket::catch(default)]
pub fn error(status: Status, _request: &Request) -> Html<String> {
    let markup = html! {
        html lang="en" {
            head {
                meta charset="utf-8";
                title { "Discord Party" }
                link rel="stylesheet" href="/static/index.css";
            }

            body.bg-background-default.text-text-body.h-screen.flex.justify-center.items-center.flex-col."gap-8" {
                h1."text-4xl".font-bold.text-text-heading { "⚠️ " (status) " ⚠️" }
                (link("/", "Back to home page"))
            }
        }
    };
    Html(markup.into_string())
}
