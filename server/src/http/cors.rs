use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::collections::HashSet;
use std::env;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let mut allow_origin = None;
        let cors_allowed_origins =
            env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "".to_owned());
        if cors_allowed_origins == "*" {
            allow_origin = Some("*");
        } else {
            let cors_allowed_origins = cors_allowed_origins
                .split(',')
                .map(str::to_owned)
                .collect::<HashSet<_>>();
            for origin in request.headers().get("Origin") {
                if cors_allowed_origins.contains(origin) {
                    allow_origin = Some(origin);
                }
            }
        }
        if let Some(origin) = allow_origin {
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
            response.set_header(Header::new("Access-Control-Allow-Methods", "*"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
    }
}
