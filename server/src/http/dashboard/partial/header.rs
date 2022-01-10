use super::DashboardContext;
use super::{h3, logo};
use maud::{html, Markup};
use rocket::uri;

pub fn header(ctx: &DashboardContext) -> Markup {
    html! {
        header.flex.items-center."gap-10".w-full."h-16".bg-background-floating.text-text-heading.font-bold {
            ."w-[350px]"."pl-8" {
                a href=(uri!("/dashboard", super::super::index::index())) { (logo()) }
            }
            (h3(html! { (ctx.title()) }))
            .ml-auto."pr-8" {
                a.flex.items-center."gap-2" href=(uri!("/auth", crate::http::auth::sign_out::sign_out())) {
                    img.rounded-full src=(ctx.user.avatar_url(5)) alt="";
                    (ctx.user.username) "#" (ctx.user.discriminator)
                }
            }
        }
    }
}
