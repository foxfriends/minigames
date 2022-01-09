use super::DashboardContext;
use maud::{html, Markup};
use rocket::uri;
use std::fmt::Display;

fn nav_link<S: Display>(ctx: &DashboardContext, href: S, title: &str) -> Markup {
    let section = ctx.section();
    let active = section == title;
    html! {
        a."px-4"."py-2".bg-blurple-default[active].text-text-body[!active]."hover:text-white".text-white[active]."rounded-sm" href=(href.to_string()) {
            (title)
        }
    }
}

pub fn nav(ctx: &DashboardContext) -> Markup {
    html! {
        nav."w-[350px]".bg-background-secondary.h-full.flex.flex-col.items-stretch."p-8"."pr-10"."shrink-0" {
            (nav_link(ctx, uri!("/dashboard", super::super::index::index()), "Dashboard"))
            (nav_link(ctx, uri!("/dashboard", super::super::admin::index::index()), "Server Admin"))
        }
    }
}
