use maud::{html, Markup};

pub fn page(children: Markup) -> Markup {
    html! {
        ."p-4" {
            (children)
        }
    }
}

pub fn full_page(children: Markup) -> Markup {
    html! {
        ."p-4".w-full.h-full {
            (children)
        }
    }
}
