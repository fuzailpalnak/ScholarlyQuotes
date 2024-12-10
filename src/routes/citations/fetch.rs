use crate::errors;
use actix_web::{web, HttpResponse};
use firebase_rs::Firebase;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub content: String,
    pub author: String,
    pub category: Option<String>,
    pub reference: String,
}

pub async fn get_random_quote_handler(
    db: web::Data<Arc<Firebase>>,
) -> Result<HttpResponse, errors::AppError> {
    let quotes_result = db
        .at("quotes")
        .get::<HashMap<String, ResponseQuote>>()
        .await;

    match quotes_result {
        Ok(quotes_map) => {
            if quotes_map.is_empty() {
                return Err(errors::AppError::NotFound("No quotes found".to_string()));
            }

            let keys: Vec<&String> = quotes_map.keys().collect();
            let random_key = keys.choose(&mut rand::thread_rng());

            if let Some(key) = random_key {
                let response = quotes_map.get(*key).ok_or_else(|| {
                    errors::AppError::NotFound("Randomly selected key not found".to_string())
                })?;
                Ok(HttpResponse::Ok().json(response))
            } else {
                Err(errors::AppError::NotFound(
                    "Could not select a random quote.".to_string(),
                ))
            }
        }
        Err(err) => Err(errors::AppError::DatabaseError(err.to_string())),
    }
}
