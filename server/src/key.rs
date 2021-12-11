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
}
