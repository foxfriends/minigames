use maud::{html, Markup, Render};

pub fn h1(children: impl Render) -> Markup {
    html! {
        h1.text-text-heading."text-2xl".font-bold {
            (children)
        }
    }
}

pub fn h3(children: impl Render) -> Markup {
    html! {
        h3.text-text-heading."text-lg".font-bold {
            (children)
        }
    }
}
