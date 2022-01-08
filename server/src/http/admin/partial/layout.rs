use maud::{html, Markup, DOCTYPE};

pub fn layout(page: Markup) -> Markup {
    html! {
        (DOCTYPE)

        head {
            meta charset="utf-8";
            link rel="stylesheet" type="text/css" href="/static/index.css";
        }

        body {
            main.flex.center.mx-auto."w-[1000px]" {
                (page)
            }
        }
    }
}
