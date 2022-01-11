use crate::game::{GameName, GameServer};
use crate::http::dashboard::DashboardContext;
use maud::{html, Markup};
use rocket::uri;

pub fn game_server_tile(ctx: &DashboardContext, server: &GameServer) -> Markup {
    let content = html! {
        .w-min.h-min."p-2".bg-background-secondary.rounded-md.flex.flex-col."gap-2" {
            ."w-28"."h-28".rounded-md.bg-background-floating.text-text-heading.flex.items-center.justify-center.font-medium.text-lg {
                (server.name()[0..1])
            }
            .truncate.text.text-text-heading.text-center."min-w-0" {
                (server.name())
            }
        }
    };
    // TODO: this is a weird hack...
    if ctx.section() == "Server Admin" && server.user_id() == ctx.user.id {
        html! {
            a."hover:shadow-xl"."hover:-translate-y-2".transition href=(uri!("/dashboard", super::super::admin::servers::edit::edit(server.name()))) {
                (content)
            }
        }
    } else {
        content
    }
}
