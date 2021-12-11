use crate::key::SPKI;
use crate::response::Response;
use rocket::response::content::Plain;

#[rocket::get("/.well-known/openid-configuration")]
pub fn get_public_key() -> Response<Plain<String>> {
    Ok(Plain(SPKI.to_owned()))
}
