use crate::http::CorsOrigin;
use std::env;
use std::path::PathBuf;

pub fn websocket_port() -> u16 {
    env::var("WEBSOCKET_PORT")
        .expect("Environment variable WEBSOCKET_PORT is required")
        .parse()
        .unwrap()
}

pub fn public_http_url() -> String {
    env::var("PUBLIC_HTTP_URL").expect("Environment variable PUBLIC_HTTP_URL is required")
}

pub fn jwt_pem() -> PathBuf {
    env::var("JWT_PEM")
        .expect("Environment variable JWT_PEM is required")
        .parse()
        .unwrap()
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("Environment variable DATABASE_URL is required")
}

pub fn discord_client_id() -> String {
    env::var("DISCORD_CLIENT_ID").expect("Environment variable DISCORD_CLIENT_ID is required")
}

pub fn discord_client_secret() -> String {
    env::var("DISCORD_CLIENT_SECRET")
        .expect("Environment variable DISCORD_CLIENT_SECRET is required")
}

pub fn cors_allowed_origins() -> CorsOrigin {
    let origins = env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|| String::from(""));
    if origins == "*" {
        return CorsOrigin::Any;
    }
    CorsOrigin::Origins(origins.split(',').map(str::to_owned).collect())
}
