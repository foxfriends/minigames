use crate::game::GameServer;
use crate::http::dashboard::DashboardContext;
use maud::{html, Markup};
use rocket::http::uri::Origin;

pub async fn game_server_tile(
    ctx: &DashboardContext,
    server: &GameServer,
    href: Option<Origin<'_>>,
) -> Markup {
    let available = server.enabled && ctx.is_available(server.name()).await;
    let content = html! {
        .w-min.h-min."p-2".bg-background-secondary.rounded-md.flex.flex-col."gap-2"."opacity-50"[!available]."hover:bg-background-hover"[href.is_some()].transition-colors {
            ."w-28"."h-28".rounded-md.bg-background-floating.text-text-heading.flex.items-center.justify-center.font-medium.text-lg {
                (server.name()[0..1])
            }
            .truncate.text.text-text-heading.text-center."min-w-0" {
                (server.name())
            }
        }
    };
    if let Some(href) = href {
        html! {
            a."hover:shadow-xl"."hover:-translate-y-2".transition href=(href) {
                (content)
            }
        }
    } else {
        content
    }
}
