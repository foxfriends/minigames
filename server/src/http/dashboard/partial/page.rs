use maud::{html, Markup};

pub fn page(children: Markup) -> Markup {
    html! {
        ."p-10" {
            (children)
        }
    }
}

pub fn full_page(children: Markup) -> Markup {
    html! {
        ."p-10".w-full.h-full {
            (children)
        }
    }
}
