use maud::{html, Markup};
use rocket::response::content::Html;

fn vsep() -> Markup {
    html! { .border-l.border-color-text-body."h-8" {} }
}

fn link(href: &str, text: &str) -> Markup {
    html! { a."text-2xl"."hover:text-text-heading" href=(href) { (text) } }
}

#[rocket::get("/")]
pub fn index() -> Html<String> {
    let markup = html! {
        html lang="en" {
            head {
                meta charset="utf-8";
                title { "Discord Party" }
                link rel="stylesheet" href="/static/index.css";
            }

            body.bg-background-default.text-text-body.h-screen.flex.justify-center.items-center.flex-col."gap-8" {
                h1."text-4xl".font-bold.text-text-heading { "Discord 🎉 Party" }
                .flex.flex-row."gap-4" {
                    (link("/install", "Install"))
                    (vsep())
                    (link("/dashboard", "Dashboard"))
                    (vsep())
                    (link("https://github.com/foxfriends/minigames", "GitHub"))
                }
            }
        }
    };
    Html(markup.into_string())
}
