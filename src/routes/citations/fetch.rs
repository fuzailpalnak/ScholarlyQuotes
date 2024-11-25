use crate::entities::quotes::Entity as Quotes;
use crate::errors;
use actix_web::{web, HttpResponse};
use log::info;
use rand::Rng;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub content: String,
    pub author: String,
    pub category: Option<String>,
    pub reference: String,
}

pub async fn fetch_quote_by_id(
    random_id: i32,
    db: &DatabaseConnection,
) -> Result<ResponseQuote, errors::AppError> {
    let random_quote = Quotes::find_by_id(random_id)
        .one(db)
        .await?
        .ok_or_else(|| errors::AppError::NotFound("Quote Not Found in DB".to_string()))?;

    Ok(ResponseQuote {
        author: random_quote.author,
        content: random_quote.content,
        category: random_quote.category,
        reference: random_quote.reference,
    })
}

pub async fn get_random_quote_handler(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<HttpResponse, errors::AppError> {
    let db_conn = db.get_ref().as_ref();
    let count = Quotes::find().count(db_conn).await?;
    if count > 0 {
        let random_id = rand::thread_rng().gen_range(1..=count as i32);
        let response = fetch_quote_by_id(random_id, db_conn).await?;
        Ok(HttpResponse::Ok().json(response))
    } else {
        info!("NO Quotes Found in the DB during fetch");
        Err(errors::AppError::NotFound(
            "No quotes found in the database.".to_string(),
        ))
    }
}
