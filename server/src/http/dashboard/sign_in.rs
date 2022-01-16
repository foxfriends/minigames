use crate::http::auth::sign_in_with_discord;
use crate::http::response::Response;
use rocket::response::Redirect;
use rocket::Request;
use std::path::PathBuf;

#[rocket::catch(401)]
pub async fn sign_in(request: &Request<'_>) -> Response<Redirect> {
    sign_in_with_discord(
        format!(
            "/dashboard/{}",
            request
                .segments::<PathBuf>(0..)
                .map_err(|_| anyhow::anyhow!("Invalid path"))?
                .display()
        ),
        request.cookies(),
    )
    .await
}
