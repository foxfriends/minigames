use maud::{html, Markup};
use std::fmt::Display;

#[allow(dead_code)]
pub fn button(children: Markup) -> Markup {
    html! {
        button.bg-blurple-default.rounded-sm.text-text-heading."px-4"."py-2"."hover:bg-blurple-hover".transition-colors {
            (children)
        }
    }
}

pub fn link_button<S: Display>(href: S, children: Markup) -> Markup {
    html! {
        a.bg-blurple-default.rounded-sm.text-text-heading."px-4"."py-2"."hover:bg-blurple-hover".transition-colors href=(href.to_string()) {
            (children)
        }
    }
}
