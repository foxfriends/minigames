use maud::{html, Markup, Render};

pub fn tt(text: impl Render) -> Markup {
    html! {
        code.break-all {
            (text)
        }
    }
}
