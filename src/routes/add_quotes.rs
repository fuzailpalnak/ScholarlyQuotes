use crate::entities::quotes;
use crate::errors;

use actix_web::{web, HttpResponse};
use log::{debug, error, info};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
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

pub async fn insert_quote(
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

#[actix_web::post("/add_quote")]
pub async fn add_quote(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<Quote>,
) -> Result<HttpResponse, errors::AppError> {
    debug!("Input data : {:?}", data);
    let quote = data.into_inner();

    match insert_quote(&db, &quote).await {
        Ok(_) => {
            info!("Data {:?} added to DB", quote);
            Ok(HttpResponse::Ok().json(quote))
        }
        Err(err) => {
            error!("Error {:?} while adding to DB", err);
            Err(err)
        }
    }
}
