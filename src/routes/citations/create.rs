use crate::entities::quotes;
use crate::errors;
use crate::utils;
use actix_web::{web, HttpRequest, HttpResponse};
use log::{debug, error, info};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    role: String,
}

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

pub async fn save_quote_to_db(
    db: &DatabaseConnection,
    quote: &Quote,
) -> Result<quotes::ActiveModel, errors::AppError> {
    let new_quote = quotes::ActiveModel {
        content: Set(quote.content.to_owned()),
        author: Set(quote.author.to_owned()),
        category: Set(quote.category.to_owned()),
        reference: Set(quote.reference.to_owned()),
        ..Default::default()
    };

    match new_quote.save(db).await {
        Ok(active_model) => Ok(active_model),
        Err(err) => Err(errors::AppError::DatabaseError(err)),
    }
}
pub async fn create_quote_handler(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<Quote>,
) -> Result<HttpResponse, errors::AppError> {
    // Process the input data
    debug!("Input data: {:?}", data);
    let quote = data.into_inner();

    // Insert the quote into the database
    save_quote_to_db(&db, &quote).await.map_err(|err| {
        error!("Error {:?} while adding to DB", err);
        err
    })?;

    info!("Data {:?} added to DB", quote);

    Ok(HttpResponse::Ok().json(quote))
}
