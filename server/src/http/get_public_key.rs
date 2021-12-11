use crate::key::JWT_KEY;
use crate::response::Response;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use rocket::response::content::Plain;

lazy_static::lazy_static! {
    static ref SPKI: String = {
        let private_key = Rsa::<Private>::private_key_from_pem(&JWT_KEY).unwrap();
        let public_key_pem = private_key.public_key_to_pem().unwrap();
        String::from_utf8(public_key_pem).unwrap()
    };
}

#[rocket::get("/.well-known/openid-configuration")]
pub fn get_public_key() -> Response<Plain<String>> {
    Ok(Plain(SPKI.to_owned()))
}
