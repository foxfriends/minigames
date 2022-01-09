use maud::{html, Markup};

pub fn h1(children: Markup) -> Markup {
    html! {
        h1.text-text-heading."text-2xl".font-bold {
            (children)
        }
    }
}

pub fn h3(children: Markup) -> Markup {
    html! {
        h3.text-text-heading."text-lg".font-bold {
            (children)
        }
    }
}
