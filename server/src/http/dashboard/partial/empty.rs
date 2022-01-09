use maud::{html, Markup};

pub fn empty(children: Markup) -> Markup {
    html! {
        .flex.items-center.justify-center.w-full."p-12".bg-background-secondary.rounded-lg {
            (children)
        }
    }
}
