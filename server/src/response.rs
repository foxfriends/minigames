use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

pub type Response<T> = Result<T, ResponseError>;

#[derive(Serialize, Deserialize)]
pub struct ResponseErrorBody {
    code: String,
    message: String,
    data: Option<serde_json::Value>,
}

pub struct ResponseError {
    status: Status,
    body: Option<Json<ResponseErrorBody>>,
}

impl<E: std::fmt::Display> From<E> for ResponseError {
    fn from(error: E) -> Self {
        Self {
            status: Status::InternalServerError,
            body: Some(Json(ResponseErrorBody {
                code: std::any::type_name::<E>().to_owned(),
                message: error.to_string(),
                data: None,
            })),
        }
    }
}

impl ResponseError {
    pub fn new(status: Status, code: String, message: String) -> Self {
        Self {
            status,
            body: Some(Json(ResponseErrorBody {
                code,
                message,
                data: None,
            })),
        }
    }

    #[allow(dead_code)]
    pub fn new_empty(status: Status) -> Self {
        Self { status, body: None }
    }

    #[allow(dead_code)]
    pub fn new_with_data<T: Serialize>(
        status: Status,
        code: String,
        message: String,
        data: T,
    ) -> Self {
        Self {
            status,
            body: Some(Json(ResponseErrorBody {
                code,
                message,
                data: Some(serde_json::to_value(data).unwrap()),
            })),
        }
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ResponseError {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let mut response = rocket::Response::build().status(self.status).finalize();
        if let Some(body) = self.body {
            response.join(body.respond_to(request)?);
        }
        Ok(response)
    }
}
