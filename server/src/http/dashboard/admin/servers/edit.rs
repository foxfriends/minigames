use crate::env::superuser_id;
use crate::game::{ApiKeys, GameName, GameServer};
use crate::http::api::v1::update_game_server;
use crate::http::cookies::UserCookie;
use crate::http::dashboard::partial::{
    button, field, info_field, layout, page, switch, text_input, tt,
};
use crate::http::dashboard::DashboardContext;
use crate::http::response::{Response, ResponseError};
use crate::postgres::PgPool;
use maud::html;
use rocket::http::Status;
use rocket::response::content::Html;
use rocket::{uri, State};

#[rocket::get("/admin/servers/<name>")]
pub async fn edit(
    db: &State<PgPool>,
    name: GameName,
    user_cookie: UserCookie<'_>,
) -> Response<Html<String>> {
    let mut conn = db.acquire().await?;
    let ctx = DashboardContext::builder(user_cookie.value())
        .await?
        .with_path(["Developer", &*name])
        .build();
    let server = match GameServer::load(&name, &mut conn).await? {
        Some(server) if ctx.user().id == server.user_id() => server,
        Some(..) => {
            return Err(ResponseError::new_empty(Status::Forbidden));
        }
        None => {
            return Err(ResponseError::new_empty(Status::NotFound));
        }
    };
    let asset = server.asset(&mut conn).await?;

    let enabled_guilds = server.list_enabled_guilds(&mut conn).await?;
    let is_supergame = match superuser_id() {
        Some(id) => id == server.user_id(),
        _ => false,
    };

    let guilds: Vec<_> = ctx
        .load_guilds()
        .await?
        .into_iter()
        .filter(|guild| guild.can_manage())
        .collect();

    let api_keys = ApiKeys::load(server.name(), &mut conn).await?;
    let markup = layout(
        &ctx,
        page(html! {
            .flex.flex-col."gap-4" {
                form.flex.flex-col."gap-4" method="POST" action=(uri!("/api/v1", update_game_server::update_game_server(server.name()))) enctype="multipart/form-data" {
                    .flex.flex-row."gap-4" {
                        (info_field(
                            "Icon",
                            html! {
                                input.hidden id="asset" type="file" accept="image/png;image/jpeg;image/gif" name="asset";
                                label.border.border-divider-dark.bg-background-secondary."p-4" for="asset" {
                                    @if let Some(asset) = asset {
                                        img.object-cover.rounded-full.bg-background-floating."w-32"."h-32" src=(asset.url());
                                    } @else {
                                        .bg-background-floating.rounded-full."w-32"."h-32"."text-2xl".flex.items-center.justify-center {
                                            (server.name().initials())
                                        }
                                    }
                                }
                            },
                        ))
                        .flex.flex-col."gap-4".grow {
                            (field(
                                "Name",
                                "name",
                                text_input("name", "", server.name()),
                            ))
                            (field(
                                "Public URL",
                                "public_url",
                                text_input("public_url", "https://my-cool-minigame.com", &server.public_url),
                            ))
                            (field(
                                "Enabled",
                                "enabled",
                                html! {
                                    .flex.items-center.justify-between {
                                        "Games that are not enabled will not be made available to play on any Discord server"
                                        (switch("enabled", server.enabled))
                                    }
                                },
                            ))
                        }
                    }

                    @if !is_supergame {
                        (info_field(
                            "Servers",
                            html! {
                                .flex.flex-col."gap-4" {
                                    "You must choose which servers to install this game to. You may only install games to servers you manage."
                                    .grid."gap-2"."grid-cols-3".bg-background-secondary."p-4".border.border-divider-dark.rounded-md {
                                        @for guild in &guilds {
                                            label.flex.flex-row."gap-4".items-center."p-2".cursor-pointer.select-none for=(guild.id) {
                                                input.hidden #(guild.id) type="checkbox" name="guilds" value=(guild.id) checked[enabled_guilds.contains(&guild.id)] autocomplete="off";
                                                .checkbox.border.border-divider-light.rounded-sm.flex.items-center.justify-center."w-6"."h-6"."shrink-0" {
                                                    .checkmark { "✓" }
                                                }
                                                @if let Some(url) = guild.icon_url(6) {
                                                    img.rounded-full."w-8"."h-8"."shrink-0" src=(url) alt="";
                                                } @else {
                                                    ."w-8"."h-8".rounded-full.bg-background-floating.flex.items-center.justify-center.text-xs."shrink-0" {
                                                        (guild.initials())
                                                    }
                                                }
                                                .font-semibold.truncate {
                                                    (guild.name)
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                        ))
                    }

                    .ml-auto { (button(html! { "Save" })) }
                }
                (info_field(
                    "Public Key",
                    tt(api_keys.public_key),
                ))
                (info_field(
                    "Secret Key",
                    tt(api_keys.secret_key),
                ))
            }
        }),
    );
    Ok(Html(markup.into_string()))
}
