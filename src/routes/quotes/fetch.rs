use crate::entities::quotes::{Column, Entity as QuoteEntity};
use crate::errors::AppError;

use actix_web::{web, HttpResponse};
use log::info;
use rand::Rng;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub content: String,
    pub author: String,
    pub reference: String,
}

async fn fetch_ids_by_language(
    db: &DatabaseConnection,
    language: &str,
) -> Result<Vec<i32>, AppError> {
    let quote_ids: Vec<i32> = QuoteEntity::find()
        .filter(Column::Language.eq(language)) // Filter by language
        .column(Column::Id) // Select only the IDs
        .all(db)
        .await?
        .into_iter()
        .map(|quote| quote.id) // Extract the IDs into a Vec
        .collect();

    Ok(quote_ids)
}

async fn fetch_random_quote_by_language(
    db: &DatabaseConnection,
    language: &str,
) -> Result<ResponseQuote, AppError> {
    let quote_ids = fetch_ids_by_language(db, language).await?;

    match quote_ids.is_empty() {
        true => Err(AppError::NotFound(
            "No quotes found in the database.".to_string(),
        )),
        false => {
            let random_id =
                rand::thread_rng().gen_range(quote_ids[0]..=quote_ids[quote_ids.len() - 1]);

            let random_quote = QuoteEntity::find_by_id(random_id)
                .one(db)
                .await?
                .ok_or_else(|| AppError::NotFound("Quote Not Found in DB".to_string()))?;
            info!("{:?}", random_quote);
            Ok(ResponseQuote {
                content: random_quote.quote,
                author: random_quote.author,
                reference: random_quote.reference.expect("Category should not be None"),
            })
        }
    }
}

// Generic function to get a quote by language
async fn get_quotes_by_language(
    db: web::Data<Arc<DatabaseConnection>>,
    language: &str,
) -> Result<HttpResponse, AppError> {
    let db_conn = db.get_ref().as_ref();
    let response = fetch_random_quote_by_language(db_conn, language).await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_quote_english(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(db, "english").await
}

pub async fn get_quote_arabic(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(db, "arabic").await
}

pub async fn get_quote_roman_urdu(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(db, "roman_urdu").await
}
