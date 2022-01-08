use maud::{html, Markup};

pub fn page(children: Markup) -> Markup {
    html! {
        ."p-4" {
            (children)
        }
    }
}
