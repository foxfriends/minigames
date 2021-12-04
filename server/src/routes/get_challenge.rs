use crate::game::Game;
use crate::postgres::PgPool;
use crate::response::{Response, ResponseError};
use crate::token::Token;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::Redirect;
use rocket::State;
use std::env;
use time::Duration;

const OAUTH_STATE_COOKIE: &str = "minigames-discord-auth-state";
const DISCORD_USER_COOKIE: &str = "minigames-discord-user";
const GAME_TOKEN_COOKIE: &str = "minigames-game-token";

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
) -> Response<Redirect> {
    let game_id = token.decode()?.game_id();
    let mut conn = db.acquire().await?;
    let _game = Game::load(game_id, &mut conn).await?;
    Ok(Redirect::to("gameurl"))
}

pub async fn start_oauth2(token: Token, cookies: &CookieJar<'_>) -> Response<Redirect> {
    let client = get_client()?.set_redirect_uri(RedirectUrl::new(format!(
        "{}/challenge",
        env::var("PUBLIC_URL")?,
    ))?);
    let (auth_url, state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_owned()))
        .url();

    let mut state_cookie = Cookie::new(OAUTH_STATE_COOKIE, state.secret().to_owned());
    state_cookie.set_http_only(true);
    state_cookie.set_secure(true);
    state_cookie.set_same_site(Some(SameSite::Strict));
    state_cookie.set_max_age(Some(Duration::seconds(60 * 5)));
    cookies.add(state_cookie);
    let mut game_cookie = Cookie::new(GAME_TOKEN_COOKIE, token.to_string());
    game_cookie.set_http_only(true);
    game_cookie.set_same_site(Some(SameSite::Strict));
    game_cookie.set_max_age(Some(Duration::seconds(60 * 5)));
    cookies.add(game_cookie);

    Ok(Redirect::to(String::from(auth_url)))
}

#[rocket::get("/challenge?<token>", rank = 1)]
pub async fn get_challenge(
    db: &State<PgPool>,
    cookies: &CookieJar<'_>,
    token: Token,
) -> Response<Redirect> {
    if let Some(cookie) = cookies.get(DISCORD_USER_COOKIE) {
        let discord_user_token = cookie.value();
        actually_get_challenge(discord_user_token.to_owned(), token, db).await
    } else {
        start_oauth2(token, cookies).await
    }
}

#[rocket::get("/challenge?<code>&<state>", rank = 2)]
pub async fn complete_oauth2(
    db: &State<PgPool>,
    cookies: &CookieJar<'_>,
    code: String,
    state: String,
) -> Response<Redirect> {
    let state_cookie = cookies.get(OAUTH_STATE_COOKIE).ok_or_else(|| {
        ResponseError::new(
            Status::Forbidden,
            "MissingState".to_owned(),
            "State cookie is missing".to_owned(),
        )
    })?;
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

    cookies.remove(Cookie::named(OAUTH_STATE_COOKIE));
    let secret = discord_user_token.access_token().secret();
    let mut user_cookie = Cookie::new(DISCORD_USER_COOKIE, secret.to_owned());
    user_cookie.set_http_only(true);
    user_cookie.set_same_site(Some(SameSite::Strict));
    user_cookie.set_max_age(
        discord_user_token
            .expires_in()
            .map(|dur| Duration::seconds(dur.as_secs() as i64)),
    );
    cookies.add(user_cookie);

    let token = Token::new(
        cookies
            .get(GAME_TOKEN_COOKIE)
            .ok_or_else(|| {
                ResponseError::new(
                    Status::NotFound,
                    "GameNotFound".to_owned(),
                    "Game cookie is missing".to_owned(),
                )
            })?
            .value()
            .to_owned(),
    );
    cookies.remove(Cookie::named(GAME_TOKEN_COOKIE));
    actually_get_challenge(secret.to_owned(), token, db).await
}
