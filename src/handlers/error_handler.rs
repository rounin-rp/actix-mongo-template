use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum Errors {
    InternalError(String),
    HttpError(HttpErrors),
}

impl ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HttpError(error) => error.status_code(),
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalError(error) => {
                let body = serde_json::to_string(&error).unwrap();
                HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(body)
            }
            Self::HttpError(error) => error.error_response(),
        }
    }
}

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum HttpErrors {
    BadRequest,
    Unauthorized,
    Message(String),
    NotFound,
}

impl ResponseError for HttpErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Message(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}
