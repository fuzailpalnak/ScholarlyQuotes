use crate::errors;
use actix_web::{web, HttpResponse};
use firebase_rs::Firebase;
use log::debug;
use serde::{Deserialize, Serialize};

use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub content: String,
    pub author: String,
    #[serde(default = "default_category")]
    pub category: Option<String>,
    pub reference: String,
}

pub fn default_category() -> Option<String> {
    Some("Uncategorized".to_string())
}

pub async fn create_quote_handler(
    db: web::Data<Arc<Firebase>>,
    data: web::Json<Quote>,
) -> Result<HttpResponse, errors::AppError> {
    debug!("Input data: {:?}", data);
    let quote = data.into_inner();

    match db.at("quotes").set(&quote).await {
        Ok(response) => return Ok(HttpResponse::Ok().json(&response.data)),
        Err(err) => return Err(errors::AppError::DatabaseError(err.to_string())),
    }
}
