use crate::response::Response;
use openssl::ec::EcKey;
use openssl::pkey::Private;

lazy_static::lazy_static! {
    static ref PRIVATE_KEY: EcKey<Private> = EcKey::private_key_from_pem(include_bytes!("../../jwt.pem")).unwrap();
}

#[rocket::get("/jwtverifier")]
pub fn get_public_key() -> Response<String> {
    let public_key = PRIVATE_KEY.public_key_to_der()?;
    let public_key = base64::encode(public_key);
    Ok(public_key)
}
