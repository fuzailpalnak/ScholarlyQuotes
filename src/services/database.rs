use crate::errors;
use crate::models;
use crate::models::LastUpdate;

use dotenv::dotenv;
use firebase_rs::Firebase;
use log::debug;
use log::info;
use rand::prelude::IteratorRandom;
use std::collections::HashMap;
use std::env;

pub async fn db_conn() -> Result<Firebase, errors::AppError> {
    dotenv().ok();
    let db_url =
        env::var("DATABASE_URL").map_err(|err| errors::AppError::DatabaseError(err.to_string()))?;

    let firebase_conn = Firebase::new(&db_url).map_err(|err| {
        errors::AppError::DatabaseError(format!("Failed to connect to Firebase: {}", err))
    })?;

    info!("Successfully connected to database.");

    Ok(firebase_conn)
}

pub async fn get_last_update_date(db: &Firebase) -> Result<String, errors::AppError> {
    let last_updated = db
        .at("last_update")
        .get::<HashMap<String, String>>()
        .await
        .map_err(|err| errors::AppError::DatabaseError(err.to_string()))?;

    let updated_date = last_updated
        .get("last_update")
        .ok_or_else(|| errors::AppError::NotFound("Randomly selected key not found".to_string()))?;

    debug!("Fetched Last Update from DB {}", updated_date);
    Ok(updated_date.clone())
}

pub async fn update_last_update_date(
    db: &Firebase,
    last_update: LastUpdate,
) -> Result<String, errors::AppError> {
    db.at("last_update")
        .update(&last_update)
        .await
        .map(|response| response.data)
        .map_err(|err| errors::AppError::DatabaseError(err.to_string()))
}

pub async fn update_quote_of_the_day(db: &Firebase) -> Result<String, errors::AppError> {
    match fetch_random_quote(db).await {
        Ok(quote) => db
            .at("quote_of_the_day")
            .at("quote")
            .update(&quote)
            .await
            .map(|response| response.data)
            .map_err(|err| errors::AppError::DatabaseError(err.to_string())),
        Err(err) => return Err(err),
    }
}

pub async fn fetch_quote_of_the_day(
    db: &Firebase,
) -> Result<models::ResponseQuote, errors::AppError> {
    let quote_map = db
        .at("quote_of_the_day")
        .get::<HashMap<String, models::ResponseQuote>>()
        .await
        .map_err(|err| errors::AppError::DatabaseError(err.to_string()))?;

    let quote = quote_map
        .get("quote")
        .ok_or_else(|| errors::AppError::DatabaseError("Quote key not found.".to_string()))?;

    Ok(quote.clone())
}

pub async fn fetch_random_quote(db: &Firebase) -> Result<models::ResponseQuote, errors::AppError> {
    let quotes_result = db
        .at("quotes")
        .get::<HashMap<String, models::ResponseQuote>>()
        .await
        .map_err(|err| errors::AppError::DatabaseError(err.to_string()))?;

    get_random_quote(quotes_result)
}

pub async fn add(db: &Firebase, quote: models::Quote) -> Result<String, errors::AppError> {
    db.at("quotes")
        .set(&quote)
        .await
        .map(|response| response.data)
        .map_err(|err| errors::AppError::DatabaseError(err.to_string()))
}

pub fn get_random_quote(
    quotes_map: HashMap<String, models::ResponseQuote>,
) -> Result<models::ResponseQuote, errors::AppError> {
    if quotes_map.is_empty() {
        return Err(errors::AppError::NotFound("No quotes found".to_string()));
    }

    let random_key = quotes_map
        .keys()
        .choose(&mut rand::thread_rng())
        .ok_or_else(|| {
            errors::AppError::NotFound("Could not select a random quote.".to_string())
        })?;

    let response = quotes_map
        .get(random_key)
        .ok_or_else(|| errors::AppError::NotFound("Randomly selected key not found".to_string()))?;

    Ok(response.clone())
}
