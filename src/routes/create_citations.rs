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
    req: HttpRequest,
) -> Result<HttpResponse, errors::AppError> {
    // Extract the token from the request
    let token = utils::jwt::extract_bearer(req)?;

    // Get the admin secret
    let admin_secret = utils::jwt::get_admin_secret()?;

    // Validate the token
    let claims = utils::jwt::validate_token(token.as_str(), admin_secret.as_str())?;
    debug!("Token claims: {:?}", claims);

    // Process the input data
    debug!("Input data: {:?}", data);
    let quote = data.into_inner();

    // Insert the quote into the database
    insert_quote(&db, &quote).await.map_err(|err| {
        error!("Error {:?} while adding to DB", err);
        err
    })?;

    info!("Data {:?} added to DB", quote);

    Ok(HttpResponse::Ok().json(quote))
}
