use super::cookies::UserCookie;
use super::response::{Response, ResponseError};
use crate::discord;
use crate::game::{Game, GameId, GameRegistry};
use crate::http::oauth::sign_in_with_discord;
use crate::postgres::PgPool;
use crate::token::Claims;
use rocket::http::{CookieJar, Status};
use rocket::response::Redirect;
use rocket::State;

#[rocket::get("/play?<game_id>", rank = 1)]
pub async fn play_game(
    db: &State<PgPool>,
    registry: &State<GameRegistry>,
    game_id: GameId,
    user_cookie: UserCookie<'_>,
) -> Response<Redirect> {
    let discord_user = discord::get_current_user(user_cookie.value()).await?;
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

#[rocket::get("/play?<game_id>", rank = 2)]
pub async fn sign_in_then_play_game(
    game_id: GameId,
    cookies: &CookieJar<'_>,
) -> Response<Redirect> {
    sign_in_with_discord(format!("/play?game_id={}", game_id), cookies).await
}
