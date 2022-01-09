use super::DashboardContext;
use super::{header, nav};
use maud::{html, Markup, DOCTYPE};

pub fn layout(ctx: &DashboardContext, page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";

                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
                link href="https://fonts.googleapis.com/css2?family=Source+Code+Pro:wght@300&display=swap" rel="stylesheet";

                link rel="stylesheet" type="text/css" href="/static/index.css";
                title { "Discord Party " (ctx.title()) }
            }

            body.bg-background-default.text-text-body.h-screen {
                (header(ctx))
                section.flex.flex-row.h-full {
                    (nav(ctx))
                    main.grow {
                        (page)
                    }
                }
            }
        }
    }
}
