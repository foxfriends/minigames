use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::env;
use std::fs::File;
use std::io::Read;

lazy_static::lazy_static! {
    pub static ref JWT_KEY: Vec<u8> = {
        let key_path = env::var("JWT_PEM").unwrap();
        let mut file = File::open(key_path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        buf
    };

    pub static ref SPKI: String = {
        let private_key = Rsa::<Private>::private_key_from_pem(&JWT_KEY).unwrap();
        let public_key_pem = private_key.public_key_to_pem().unwrap();
        String::from_utf8(public_key_pem).unwrap()
    };
}
