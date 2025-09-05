use actix_web::error::PayloadError;
use actix_web::{HttpResponse, ResponseError};
use argon2::password_hash::Error as Argon2Error;
use jsonwebtoken::errors::Error as JwtError;
use openssl::error::ErrorStack;
use sqlx::Error as SqlxError;
use std::error::Error as StdError;
use std::{fmt, sync::PoisonError};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ErrorRes {
    status: String,
    data: &'static str,
}

#[derive(Debug)]
pub enum Error {
    Conflict(String),
    Unauthorized(String),
    BadRequest(String),
    Internal(String),
    UniqueNameViolation(String),
    NotFound(),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Error::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Error::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::UniqueNameViolation(msg) => write!(f, "Unique constraint violation: {}", msg),
            Error::NotFound() => write!(f, "Not Found"),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Conflict(msg) => HttpResponse::Conflict().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::Unauthorized(msg) => HttpResponse::Unauthorized().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::BadRequest(msg) => HttpResponse::BadRequest().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::Internal(msg) => HttpResponse::InternalServerError().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::UniqueNameViolation(msg) => HttpResponse::Conflict().json(ErrorRes {
                status: msg.to_string(),
                data: "",
            }),
            Error::NotFound() => HttpResponse::NotFound().finish(),
        }
    }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Conflict(msg) => std::io::Error::new(std::io::ErrorKind::Other, msg),
            Error::Unauthorized(msg) => {
                std::io::Error::new(std::io::ErrorKind::PermissionDenied, msg)
            }
            Error::BadRequest(msg) => std::io::Error::new(std::io::ErrorKind::InvalidInput, msg),
            Error::Internal(msg) => std::io::Error::new(std::io::ErrorKind::Other, msg),
            Error::UniqueNameViolation(msg) => {
                std::io::Error::new(std::io::ErrorKind::AlreadyExists, msg)
            }
            Error::NotFound() => {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Resource not found")
            }
        }
    }
}

impl From<PayloadError> for Error {
    fn from(err: PayloadError) -> Self {
        Error::Internal(format!("Payload error: {}", err))
    }
}

impl From<Box<dyn StdError>> for Error {
    fn from(err: Box<dyn StdError>) -> Self {
        Error::Internal(format!(
            "Some crate handlers dont like telling whats the problem: Dynamic error: {}",
            err
        ))
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Error::Internal(format!("Multithread error, POISONED: {}", err))
    }
}

impl From<ErrorStack> for Error {
    fn from(err: ErrorStack) -> Self {
        Error::Internal(format!("OpenSSL error: {}", err))
    }
}

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        Error::Internal(format!("DB Connection error: {}", err))
    }
}

impl From<JwtError> for Error {
    fn from(err: JwtError) -> Self {
        Error::Internal(format!("JWT encoding error: {}", err))
    }
}

impl From<Argon2Error> for Error {
    fn from(err: Argon2Error) -> Self {
        Error::Internal(format!("Crypto hash error: {}", err))
    }
}
