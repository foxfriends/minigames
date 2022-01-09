use super::logo;
use super::DashboardContext;
use maud::{html, Markup};

pub fn header(ctx: &DashboardContext) -> Markup {
    html! {
        header.flex.items-center."gap-4".w-full."h-16".bg-background-floating.text-text-heading.font-bold {
            ."w-[350px]"."pl-8" {
                a href="/dashboard" { (logo()) }
            }
            .text-lg { (ctx.title) }
            .ml-auto."pr-8" {
                a.flex.items-center."gap-2" href="/sign-out" {
                    img.rounded-full src=(ctx.user.avatar_url(5)) alt="";
                    (ctx.user.username) "#" (ctx.user.discriminator)
                }
            }
        }
    }
}
