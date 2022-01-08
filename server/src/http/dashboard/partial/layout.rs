use super::DashboardContext;
use super::{header, nav};
use maud::{html, Markup, DOCTYPE};

pub fn layout(ctx: &DashboardContext, page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                link rel="stylesheet" type="text/css" href="/static/index.css";
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
