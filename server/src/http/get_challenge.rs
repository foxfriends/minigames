use crate::cookies::{GameCookie, StateCookie, UserCookie};
use crate::game::{Game, GameRegistry};
use crate::postgres::PgPool;
use crate::response::{Response, ResponseError};
use crate::token::Token;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::State;
use std::env;

fn get_client() -> anyhow::Result<BasicClient> {
    Ok(BasicClient::new(
        ClientId::new(env::var("DISCORD_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("DISCORD_CLIENT_SECRET")?)),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_owned())?,
        Some(TokenUrl::new(
            "https://discord.com/api/oauth2/token".to_owned(),
        )?),
    ))
}

pub async fn actually_get_challenge(
    _discord_user_token: String,
    token: Token,
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
) -> Response<Redirect> {
    let game_id = token.decode()?.game_id();
    let mut conn = db.acquire().await?;
    let game = Game::load(game_id, &mut conn).await?;
    if let Some(url) = registry.locate(&game.game).await {
        Ok(Redirect::to(format!("{}/?token={}", url, token)))
    } else {
        Err(ResponseError::new(
            Status::NotFound,
            "GameNotRegistered".to_owned(),
            format!("Server for {} could not be located", game.game),
        ))
    }
}

pub async fn start_oauth2(token: Token, cookies: &CookieJar<'_>) -> Response<Redirect> {
    let client = get_client()?.set_redirect_uri(RedirectUrl::new(format!(
        "{}/challenge",
        env::var("PUBLIC_HTTP_URL")?,
    ))?);
    let (auth_url, state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_owned()))
        .url();

    StateCookie::add_to(cookies, state.secret().to_owned());
    GameCookie::add_to(cookies, token);
    Ok(Redirect::to(String::from(auth_url)))
}

#[rocket::get("/challenge?<token>", rank = 1)]
pub async fn get_challenge<'r>(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    cookies: &CookieJar<'r>,
    token: Token,
    user_cookie: Option<UserCookie<'r>>,
) -> Response<Redirect> {
    if let Some(discord_user_token) = user_cookie {
        actually_get_challenge(discord_user_token.value().to_owned(), token, db, registry).await
    } else {
        start_oauth2(token, cookies).await
    }
}

#[rocket::get("/challenge?<code>&<state>", rank = 2)]
pub async fn complete_oauth2<'r>(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    cookies: &CookieJar<'r>,
    code: String,
    state: String,
    state_cookie: StateCookie<'r>,
    game_cookie: GameCookie,
) -> Response<Redirect> {
    if state_cookie.value() != state {
        return Err(ResponseError::new(
            Status::Forbidden,
            "InvalidState".to_owned(),
            "State cookie is invalid".to_owned(),
        ));
    }

    let client = get_client()?;
    let discord_user_token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?;

    UserCookie::add_to(cookies, &discord_user_token);
    StateCookie::remove_from(cookies);
    GameCookie::remove_from(cookies);
    let secret = discord_user_token.access_token().secret().to_owned();
    actually_get_challenge(secret, game_cookie.value().clone(), db, registry).await
}
