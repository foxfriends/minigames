use crate::game::Game;
use crate::postgres::PgPool;
use crate::response::{Response, ResponseError};
use crate::token::Token;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rocket::form::FromForm;
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::Redirect;
use rocket::State;
use std::env;
use time::Duration;

const OAUTH_STATE_COOKIE: &str = "minigames-discord-auth-state";
const DISCORD_USER_COOKIE: &str = "minigames-discord-user";

#[derive(FromForm)]
pub struct GetChallenge {
    token: Token,
    state: Option<String>,
    code: Option<String>,
}

pub async fn actually_get_challenge(
    _discord_user_token: String,
    db: &State<PgPool>,
    params: GetChallenge,
) -> Response<Redirect> {
    let game_id = params.token.decode()?.game_id();
    let mut conn = db.acquire().await?;
    let _game = Game::load(game_id, &mut conn).await?;
    Ok(Redirect::to("gameurl"))
}

pub async fn complete_oauth2(
    code: String,
    db: &State<PgPool>,
    cookies: &CookieJar<'_>,
    params: GetChallenge,
) -> Response<Redirect> {
    let state_cookie = cookies.get_private(DISCORD_USER_COOKIE).ok_or_else(|| {
        ResponseError::new(
            Status::Forbidden,
            "InvalidState".to_owned(),
            "State cookie is missing".to_owned(),
        )
    })?;
    let state_param = params.state.as_ref().ok_or_else(|| {
        ResponseError::new(
            Status::BadRequest,
            "MissingParameter".to_owned(),
            "Query parameter `state` is missing".to_owned(),
        )
    })?;
    if state_cookie.value() != state_param {
        return Err(ResponseError::new(
            Status::Forbidden,
            "InvalidState".to_owned(),
            "State cookie is missing".to_owned(),
        ));
    }

    let client = get_client()?;
    let discord_user_token = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?;

    cookies.remove_private(Cookie::named(OAUTH_STATE_COOKIE));
    let secret = discord_user_token.access_token().secret();
    let mut user_cookie = Cookie::new(DISCORD_USER_COOKIE, secret.to_owned());
    user_cookie.set_http_only(true);
    user_cookie.set_secure(true);
    user_cookie.set_same_site(Some(SameSite::Strict));
    user_cookie.set_max_age(
        discord_user_token
            .expires_in()
            .map(|dur| Duration::seconds(dur.as_secs() as i64)),
    );
    cookies.add_private(user_cookie);
    actually_get_challenge(secret.to_owned(), db, params).await
}

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

pub async fn start_oauth2(cookies: &CookieJar<'_>, params: GetChallenge) -> Response<Redirect> {
    let client = get_client()?.set_redirect_uri(RedirectUrl::new(format!(
        "{}/challenge?token={}",
        env::var("PUBLIC_URL")?,
        params.token.to_string()
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
    cookies.add_private(state_cookie);

    Ok(Redirect::to(String::from(auth_url)))
}

#[rocket::get("/challenge?<params>")]
pub async fn get_challenge(
    db: &State<PgPool>,
    cookies: &CookieJar<'_>,
    mut params: GetChallenge,
) -> Response<Redirect> {
    if let Some(cookie) = cookies.get(DISCORD_USER_COOKIE) {
        let discord_user_token = cookie.value();
        actually_get_challenge(discord_user_token.to_owned(), db, params).await
    } else if let Some(code) = params.code.take() {
        complete_oauth2(code, db, cookies, params).await
    } else {
        start_oauth2(cookies, params).await
    }
}
