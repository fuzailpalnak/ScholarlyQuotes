use crate::errors;
use crate::services::database;

use actix_web::{web, HttpResponse};
use firebase_rs::Firebase;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub content: String,
    pub author: String,
    pub category: Option<String>,
    pub reference: String,
}

pub async fn get_random_quote_handler(
    db_conn: web::Data<Arc<Firebase>>,
) -> Result<HttpResponse, errors::AppError> {
    let quotes_result = database::fetch_random_quote(&db_conn).await;

    match quotes_result {
        Ok(quote) => Ok(HttpResponse::Ok().json(quote)),
        Err(err) => Err(errors::AppError::DatabaseError(err.to_string())),
    }
}

pub async fn quote_of_the_day_handler(
    db_conn: web::Data<Arc<Firebase>>,
) -> Result<HttpResponse, errors::AppError> {
    let quote_result = database::fetch_quote_of_the_day(&db_conn).await;

    match quote_result {
        Ok(quote) => Ok(HttpResponse::Ok().json(quote)),
        Err(err) => Err(errors::AppError::DatabaseError(err.to_string())),
    }
}
