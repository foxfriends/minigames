use maud::{html, Markup};

pub fn logo() -> Markup {
    html! {
        .flex.items-center."text-lg" {
            img."mr-2"."h-8" src="/static/discord-logo.svg" alt="Discord";
            "🎉 Party"
        }
    }
}
