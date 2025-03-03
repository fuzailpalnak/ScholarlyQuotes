use crate::db_queries::{fetch, model, qotd};
use crate::errors::AppError;
use crate::utils;
use crate::AppState;
use actix_web::{web, HttpResponse};
use redis::AsyncCommands;
use sea_orm::DatabaseConnection;

async fn get_quote_from_cache_or_db(
    db_conn: &DatabaseConnection,
    redis_conn: &mut redis::aio::Connection,
    language: &str,
) -> Result<model::ResponseQuote, AppError> {
    let key = format!("qotd:{}", language);

    match redis_conn.get::<_, String>(key.clone()).await {
        Ok(cached_quote) => match serde_json::from_str::<model::ResponseQuote>(&cached_quote) {
            Ok(quote) => {
                log::info!(
                    "Successfully fetched quote from Redis for language: {}",
                    language
                );
                Ok(quote)
            }
            Err(e) => {
                log::error!("Failed to deserialize quote from Redis: {}", e);
                qotd::fetch_and_cache_quote_from_db(db_conn, redis_conn, language).await
            }
        },
        Err(e) => {
            log::error!("Error fetching quote from Redis: {}", e);
            qotd::fetch_and_cache_quote_from_db(db_conn, redis_conn, language).await
        }
    }
}

pub async fn fetch_qotd_by_language(
    db_conn: &DatabaseConnection,
    redis: &redis::Client,
    language: &str,
) -> Result<model::ResponseQuote, AppError> {
    let mut redis_conn = redis.get_async_connection().await?;

    match get_quote_from_cache_or_db(db_conn, &mut redis_conn, language).await {
        Ok(quote) => Ok(quote),
        Err(e) => Err(e),
    }
}

async fn get_quotes_by_language(
    app_state: web::Data<AppState>,
    language: &str,
) -> Result<HttpResponse, AppError> {
    let db_conn = app_state.db.as_ref();
    let response = fetch::fetch_random_quote_by_language(db_conn, language).await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_quote_english(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(app_state, utils::languages::Language::English.as_str()).await
}

pub async fn get_quote_arabic(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(app_state, utils::languages::Language::Arabic.as_str()).await
}

pub async fn get_quote_roman_urdu(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(app_state, utils::languages::Language::RomanUrdu.as_str()).await
}

pub async fn get_qotd_english(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db_conn = app_state.db.as_ref();
    let redis_client = app_state.redis_client.as_ref();
    let response = fetch_qotd_by_language(
        db_conn,
        redis_client,
        utils::languages::Language::English.as_str(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_qotd_roman_urdu(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db_conn = app_state.db.as_ref();
    let redis_client = app_state.redis_client.as_ref();
    let response = fetch_qotd_by_language(
        db_conn,
        redis_client,
        utils::languages::Language::RomanUrdu.as_str(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(response))
}
