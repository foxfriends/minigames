use crate::game::Game;
use crate::postgres::PgPool;
use crate::response::Response;
use crate::token::Token;
use rocket::form::FromForm;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::State;
use std::env;

const OAUTH_STATE_COOKIE: &str = "minigames-discord-auth-state";

#[derive(FromForm)]
pub struct GetChallenge {
    token: Token,
}

#[rocket::get("/challenge?<params>")]
pub async fn get_challenge(
    db: &State<PgPool>,
    cookies: &CookieJar<'_>,
    params: GetChallenge,
) -> Response<Redirect> {
    let game_id = params.token.decode()?.game_id();

    let state = "hello_world";
    cookies.add_private(Cookie::new(OAUTH_STATE_COOKIE, state));

    let mut conn = db.acquire().await?;
    let _game = Game::load(game_id, &mut conn).await?;
    let client_id = env::var("DISCORD_CLIENT_ID")?;

    Ok(Redirect::to(format!("https://discord.com/api/oauth2/authorize?response_type=code&client_id={}&scope=identify&state={}", client_id, state)))
}
