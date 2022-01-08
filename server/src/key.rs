use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::fs::File;
use std::io::Read;

lazy_static::lazy_static! {
    pub static ref JWT_KEY: Vec<u8> = {
        let key_path = crate::env::jwt_pem();
        let mut file = File::open(key_path).expect("File indicated by JWT_PEM could not be found");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Failed to read JWT_PEM file");
        buf
    };

    pub static ref SPKI: String = {
        let private_key = Rsa::<Private>::private_key_from_pem(&JWT_KEY).expect("Failed to read private key from JWT_PEM");
        let public_key_pem = private_key.public_key_to_pem().expect("Failed to extract public key from JWT_PEM");
        String::from_utf8(public_key_pem).unwrap()
    };
}
