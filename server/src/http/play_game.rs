use super::cookies::{GameCookie, StateCookie, UserCookie};
use super::response::{Response, ResponseError};
use crate::discord;
use crate::game::{Game, GameId, GameRegistry};
use crate::postgres::PgPool;
use crate::token::Claims;
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
    )
    .set_redirect_uri(RedirectUrl::new(format!(
        "{}/play",
        env::var("PUBLIC_HTTP_URL")?,
    ))?))
}

pub async fn actually_play_game(
    discord_user_token: String,
    game_id: GameId,
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
) -> Response<Redirect> {
    let discord_user = discord::get_current_user(discord_user_token).await?;
    let mut conn = db.acquire().await?;
    let game = Game::load(game_id, &mut conn).await?;
    let user_token = Claims::new(game.game.clone(), discord_user.id).sign()?;
    if let Some(url) = registry.locate(&game.game).await {
        Ok(Redirect::to(format!(
            "{}?game_id={}&token={}",
            url, game_id, user_token
        )))
    } else {
        Err(ResponseError::new(
            Status::NotFound,
            "GameNotRegistered".to_owned(),
            format!("Server for {} could not be located", game.game),
        ))
    }
}

pub async fn start_oauth2(game_id: GameId, cookies: &CookieJar<'_>) -> Response<Redirect> {
    let client = get_client()?;
    let (auth_url, state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_owned()))
        .url();

    StateCookie::add_to(cookies, state.secret().to_owned());
    GameCookie::add_to(cookies, game_id);
    Ok(Redirect::to(String::from(auth_url)))
}

#[rocket::get("/play?<game_id>", rank = 1)]
pub async fn play_game<'r>(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    cookies: &CookieJar<'r>,
    game_id: GameId,
    user_cookie: Option<UserCookie<'r>>,
) -> Response<Redirect> {
    if let Some(discord_user_token) = user_cookie {
        actually_play_game(discord_user_token.value().to_owned(), game_id, db, registry).await
    } else {
        start_oauth2(game_id, cookies).await
    }
}

#[rocket::get("/play?<code>&<state>", rank = 2)]
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
        .add_extra_param("grant_type", "authorization_code")
        .request_async(async_http_client)
        .await?;

    UserCookie::add_to(cookies, &discord_user_token);
    StateCookie::remove_from(cookies);
    GameCookie::remove_from(cookies);
    let secret = discord_user_token.access_token().secret().to_owned();
    actually_play_game(secret, game_cookie.value(), db, registry).await
}
