use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::collections::HashSet;

pub struct Cors;

pub enum CorsOrigin {
    Origins(HashSet<String>),
    Any,
}

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
        match crate::env::cors_allowed_origins() {
            CorsOrigin::Any => {
                allow_origin = Some("*");
            }
            CorsOrigin::Origins(origins) => {
                for origin in request.headers().get("Origin") {
                    if origins.contains(origin) {
                        allow_origin = Some(origin);
                    }
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
