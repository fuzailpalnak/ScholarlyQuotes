use actix_web::{HttpResponse, ResponseError};
use log::error;
use redis::RedisError;
use sea_orm::DbErr;
use serde_json::Error as SerdeError;
use std::{fmt, io::Error as IOError, time::SystemTimeError};

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DbErr),
    ActixError(actix_web::Error),
    IOError(IOError),
    NotFound(String),
    SystemTimeError(SystemTimeError),
    RedisError(RedisError),
    SerdeError(SerdeError),
    ApiKeyError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::ActixError(e) => write!(f, "Actix error: {}", e),
            AppError::IOError(e) => write!(f, "I/O error: {}", e),
            AppError::NotFound(msg) => write!(f, "Resource not found: {}", msg),
            AppError::SystemTimeError(e) => write!(f, "System time error: {}", e),
            AppError::RedisError(e) => write!(f, "Redis error: {}", e),
            AppError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            AppError::ApiKeyError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        error!("Error occurred: {}", self);

        match self {
            AppError::DatabaseError(_)
            | AppError::ActixError(_)
            | AppError::IOError(_)
            | AppError::RedisError(_)
            | AppError::SerdeError(_) => HttpResponse::InternalServerError().json({
                serde_json::json!({"error": "Internal Server Error", "message": self.to_string()})
            }),
            AppError::ApiKeyError(_) => HttpResponse::Forbidden()
                .json(serde_json::json!({"error": "Forbidden", "message": self.to_string()})),
            AppError::NotFound(_) => HttpResponse::NotFound()
                .json(serde_json::json!({"error": "Not Found", "message": self.to_string()})),
            AppError::SystemTimeError(_) => HttpResponse::BadRequest()
                .json(serde_json::json!({"error": "Bad Request", "message": self.to_string()})),
        }
    }
}

impl From<SerdeError> for AppError {
    fn from(e: SerdeError) -> Self {
        AppError::SerdeError(e)
    }
}

impl From<RedisError> for AppError {
    fn from(e: RedisError) -> Self {
        AppError::RedisError(e)
    }
}

impl From<SystemTimeError> for AppError {
    fn from(e: SystemTimeError) -> Self {
        AppError::SystemTimeError(e)
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
