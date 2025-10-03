use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal server error!: {0}")]
    Internal(String),

    #[error("Bad request!: {0}")]
    BadRequest(String),

    #[error("Unauthorized!: {0}")]
    Unauthorized(String),

    #[error("Too many requests!: {0}")]
    Conflict(String),

    #[error("Too many requests!: {0}")]
    TooManyRequests(String),

    #[error("Not found")]
    NotFound(),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Conflict(msg) => HttpResponse::Conflict().body(msg.to_string()),
            Error::Unauthorized(msg) => HttpResponse::Unauthorized().body(msg.to_string()),
            Error::BadRequest(msg) => HttpResponse::BadRequest().body(msg.to_string()),
            Error::Internal(msg) => HttpResponse::InternalServerError().body(msg.to_string()),
            Error::TooManyRequests(msg) => HttpResponse::TooManyRequests().body(msg.to_string()),
            Error::NotFound() => HttpResponse::NotFound().finish(),
        }
    }
}

impl From<crypto::Error> for Error {
    fn from(err: crypto::Error) -> Self {
        match err {
            crypto::Error::InvalidCipthertextError(e) => Error::Unauthorized(e),
            crypto::Error::InvalidSignitureError(e) => Error::Unauthorized(e),
            crypto::Error::DecryptionError(e) => Error::Unauthorized(e),
            _ => Error::Internal(err.to_string()),
        }
    }
}

impl From<auth::Error> for Error {
    fn from(err: auth::Error) -> Self {
        match err {
            auth::Error::InvalidJWT(e) => Error::Unauthorized(e),
            auth::Error::InvalidRefreshToken(e) => Error::Unauthorized(e),
            // auth::Error::InvalidCredentials(e) => Error::BadRequest(e),
            // auth::Error::InvalidAccount(e) => Error::BadRequest(e),
            _ => Error::Internal(err.to_string()),
        }
    }
}

impl From<account::Error> for Error {
    fn from(err: account::Error) -> Self {
        match err {
            account::Error::InvalidCredentials(e) => Error::BadRequest(e),
            account::Error::InvalidAccount(e) => Error::BadRequest(e),
            account::Error::FriendRequestDoesntExist() => {
                Error::BadRequest("Friend request does not exist!".into())
            }
            _ => Error::Internal(err.to_string()),
        }
    }
}
