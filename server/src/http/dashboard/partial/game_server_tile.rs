use crate::game::GameServer;
use crate::http::dashboard::DashboardContext;
use maud::{html, Markup};

pub async fn game_server_tile(ctx: &DashboardContext, server: &GameServer) -> Markup {
    let available = server.enabled && ctx.is_available(server.name()).await;
    html! {
        .w-min.h-min."p-2".bg-background-secondary.rounded-md.flex.flex-col."gap-2"."opacity-50"[!available] {
            ."w-28"."h-28".rounded-md.bg-background-floating.text-text-heading.flex.items-center.justify-center.font-medium.text-lg {
                (server.name()[0..1])
            }
            .truncate.text.text-text-heading.text-center."min-w-0" {
                (server.name())
            }
        }
    }
}
