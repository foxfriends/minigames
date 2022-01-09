use crate::http::cookies::{RedirectCookie, StateCookie, UserCookie};
use crate::http::response::{Response, ResponseError};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;

fn get_client() -> anyhow::Result<BasicClient> {
    Ok(BasicClient::new(
        ClientId::new(crate::env::discord_client_id()),
        Some(ClientSecret::new(crate::env::discord_client_secret())),
        AuthUrl::new("https://discord.com/api/oauth2/authorize".to_owned())?,
        Some(TokenUrl::new(
            "https://discord.com/api/oauth2/token".to_owned(),
        )?),
    )
    .set_redirect_uri(RedirectUrl::new(format!(
        "{}/oauth2",
        crate::env::public_http_url(),
    ))?))
}

pub async fn sign_in_with_discord(
    redirect_path: String,
    cookies: &CookieJar<'_>,
) -> Response<Redirect> {
    let client = get_client()?;
    let (auth_url, state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_owned()))
        .url();
    RedirectCookie::add_to(cookies, redirect_path);
    StateCookie::add_to(cookies, state.secret().to_owned());
    Ok(Redirect::to(String::from(auth_url)))
}

#[rocket::get("/oauth2?<code>&<state>")]
pub async fn complete_oauth2<'r>(
    cookies: &CookieJar<'r>,
    code: String,
    state: String,
    state_cookie: StateCookie<'r>,
    redirect_cookie: RedirectCookie<'r>,
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
    RedirectCookie::remove_from(cookies);
    Ok(Redirect::to(redirect_cookie.value().to_owned()))
}
