use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::errors::Error as JWTError;
use log::error;
use sea_orm::DbErr;
use serde_json::Error as SerdeError;
use std::io::Error as IOError;
use std::time::SystemTimeError;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DbErr),
    ActixError(actix_web::Error),
    IOError(IOError),
    NotFound(String),
    Unauthorized(String),
    SystemTimeError(SystemTimeError),
    TokenError(JWTError),
    RedisError(redis::RedisError),
    SerdeError(SerdeError),
}

impl From<SerdeError> for AppError {
    fn from(e: SerdeError) -> Self {
        AppError::SerdeError(e)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        AppError::RedisError(e)
    }
}

impl From<SystemTimeError> for AppError {
    fn from(e: SystemTimeError) -> Self {
        AppError::SystemTimeError(e)
    }
}

impl From<JWTError> for AppError {
    fn from(e: JWTError) -> Self {
        AppError::TokenError(e)
    }
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        AppError::DatabaseError(e)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(e: actix_web::Error) -> Self {
        AppError::ActixError(e)
    }
}

impl From<IOError> for AppError {
    fn from(e: IOError) -> Self {
        AppError::IOError(e)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::ActixError(e) => write!(f, "Actix error: {}", e),
            AppError::IOError(e) => write!(f, "I/O error: {}", e),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized request: {}", msg),
            AppError::SystemTimeError(e) => write!(f, "System error: {:?}", e),
            AppError::TokenError(e) => write!(f, "Token error: {:?}", e),
            AppError::RedisError(e) => write!(f, "Redis error: {:?}", e),
            AppError::SerdeError(e) => write!(f, "Json Conversion error: {:?}", e),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("{}", self);

        match self {
            AppError::TokenError(_) => {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", self))
            }
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", self))
            }
            AppError::ActixError(_) => {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", self))
            }
            AppError::IOError(_) => {
                HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", self))
            }
            AppError::NotFound(_) => HttpResponse::NotFound().body(format!("Not Found: {}", self)),
            AppError::SystemTimeError(_) => {
                HttpResponse::BadRequest().body(format!("Not Found: {}", self))
            }
            AppError::Unauthorized(_) => {
                HttpResponse::Unauthorized().body(format!("Unauthorized Request: {}", self))
            }
            AppError::RedisError(e) => {
                HttpResponse::InternalServerError().body(format!("Redis error: {:?}", e))
            }
            AppError::SerdeError(e) => {
                HttpResponse::InternalServerError().body(format!("Json Conversion error: {:?}", e))
            }
        }
    }
}
