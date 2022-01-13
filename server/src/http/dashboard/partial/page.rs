use super::h1;
use maud::{html, Markup, Render};

pub fn page(children: Markup) -> Markup {
    html! {
        ."p-10" {
            (children)
        }
    }
}

#[allow(dead_code)]
pub fn full_page(children: Markup) -> Markup {
    html! {
        ."p-10".w-full.h-full {
            (children)
        }
    }
}

pub fn page_heading(title: impl Render, actions: Option<Markup>) -> Markup {
    html! {
        .flex.items-center.justify-between.w-full {
            (h1(title))
            @if let Some(actions) = actions {
                (actions)
            }
        }
    }
}
