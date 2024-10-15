use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[allow(dead_code)]
#[derive(Debug, Display)]
#[non_exhaustive]
pub enum ServiceError {
    #[display("Internal Server Error")]
    InternalServerError,
    #[display("BadRequest: {_0}")]
    BadRequest(String),
    #[display("Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().finish(),
            ServiceError::BadRequest(ref _message) => HttpResponse::BadRequest().finish(),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}
