use crate::discord::UserDiscordGuild;
use crate::guild::GuildId;
use maud::{html, Markup};
use rocket::uri;

pub fn guild_row(server: &UserDiscordGuild) -> Markup {
    html! {
        a."hover:shadow-xl"."hover:-translate-y-2".transition href=(uri!("/dashboard", super::super::library::library(&server.id))) {
            .flex.flex-row."gap-4".items-center.bg-background-secondary.rounded-md."p-4"."hover:bg-background-hover".transition-colors {
                @if let Some(url) = server.icon_url(6) {
                    img.rounded-full."w-8"."h-8"."shrink-0" src=(url) alt="";
                } @else {
                    ."w-8"."h-8".rounded-full.bg-background-floating.flex.items-center.justify-center.text-xs."shrink-0" {
                        (server.initials())
                    }
                }
                .font-semibold.truncate {
                    (server.name)
                }
            }
        }
    }
}
