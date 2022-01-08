use super::DashboardContext;
use maud::{html, Markup};

fn nav_link(ctx: &DashboardContext, href: &str, title: &str) -> Markup {
    html! {
        a."px-4"."py-2".bg-blurple[ctx.title == title].text-text-body[ctx.title != title]."hover:text-white".text-white[ctx.title == title]."rounded-sm" href=(href) {
            (title)
        }
    }
}

pub fn nav(ctx: &DashboardContext) -> Markup {
    html! {
        nav."w-[350px]".bg-background-secondary.h-full.flex.flex-col.items-stretch."p-8"."pr-10" {
            (nav_link(ctx, "/dashboard", "Dashboard"))
            (nav_link(ctx, "/dashboard/admin", "Server Admin"))
        }
    }
}
