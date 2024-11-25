use crate::errors;
use crate::utils;
use actix_web::HttpResponse;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub content: String,
    pub author: String,
    pub category: Option<String>,
    pub reference: String,
}

pub async fn generate_token_handler() -> Result<HttpResponse, errors::AppError> {
    let admin_secret = utils::env_utils::load_admin_secret().unwrap();
    match utils::jwt_utils::generate_token_with_role("client", &admin_secret, 3600) {
        Ok(token) => Ok(HttpResponse::Ok().json(token)),
        Err(err) => Err(err),
    }
}
