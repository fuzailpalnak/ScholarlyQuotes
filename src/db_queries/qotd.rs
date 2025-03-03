use crate::db_queries::fetch::fetch_random_quote_by_language;
use crate::entities::quote_of_the_day::{self, Column as QOTDColumn, Entity as QOTDEntity};
use crate::errors::AppError;
use chrono::Utc;

use super::model::ResponseQuote;
use crate::entities::quotes::Entity as QuoteEntity;
use log::info;
use redis::AsyncCommands;
use sea_orm::sea_query::OnConflict;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::utils;
use serde_json::Error as SerdeError;

pub async fn qotd_db_insert(
    db_conn: &DatabaseConnection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let today = Utc::now().date_naive();
    let conflict = OnConflict::columns([QOTDColumn::Language, QOTDColumn::Date])
        .do_nothing()
        .clone();
    let _ = QOTDEntity::insert(quote_of_the_day::ActiveModel {
        language: Set(quote.language.to_string()),
        quote_id: Set(quote.id),
        date: Set(today),
        ..Default::default()
    })
    .on_conflict(conflict)
    .exec(db_conn)
    .await?;

    Ok(())
}

pub async fn qotd_redis_insert(
    redis: &redis::Client,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let mut conn = redis
        .get_async_connection()
        .await
        .map_err(|err| AppError::from(err))?;

    cache_qotd(&mut conn, quote).await?;
    update_last_qotd_time(&mut conn, quote).await?;

    Ok(())
}

pub async fn cache_qotd(
    conn: &mut redis::aio::Connection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let key = format!("qotd:{}", quote.language);
    info!("Caching QOTD: {:?}", quote);

    let quote_json =
        serde_json::to_string(quote).map_err(|err: SerdeError| AppError::SerdeError(err))?;
    let _: () = conn
        .set_ex(key, quote_json, 86400)
        .await
        .map_err(|err| AppError::RedisError(err))?; // Expire in 24 hours
    Ok(())
}

pub async fn update_last_qotd_time(
    conn: &mut redis::aio::Connection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let key = format!("qotd:last_update:{}", quote.language);
    let current_time = Utc::now().timestamp();

    let _: () = conn.set(key, current_time).await?;
    Ok(())
}

pub async fn set_qotd(db_conn: &DatabaseConnection, redis: &redis::Client) -> Result<(), AppError> {
    for lang in utils::languages::Language::variants() {
        let response = fetch_random_quote_by_language(db_conn, lang.as_str()).await?;
        qotd_db_insert(db_conn, &response).await?;
        qotd_redis_insert(redis, &response).await?;
    }

    Ok(())
}

pub async fn get_redis_last_update(
    redis_client: &redis::Client,
    lang: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut conn = redis_client.get_async_connection().await?;
    let key = format!("qotd:last_update:{}", lang);
    let last_update: Result<i64, _> = conn.get(key).await;
    info!("Last Update Timestamp: {:?}", last_update);

    match last_update {
        Ok(timestamp) => Ok(timestamp),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn fetch_and_cache_quote_from_db(
    db_conn: &DatabaseConnection,
    redis_conn: &mut redis::aio::Connection,
    language: &str,
) -> Result<ResponseQuote, AppError> {
    let qotd = QOTDEntity::find()
        .filter(QOTDColumn::Language.eq(language))
        .find_also_related(QuoteEntity)
        .one(db_conn)
        .await?;

    match qotd {
        Some((_, Some(quote))) => {
            let response_quote = ResponseQuote {
                id: quote.id,
                content: quote.quote,
                author: quote.author,
                reference: quote.reference.unwrap_or_else(|| "Unknown".to_string()),
                language: quote.language,
            };

            let quote_json = serde_json::to_string(&response_quote).map_err(|e| {
                log::error!("Error serializing quote: {}", e);
                AppError::SerdeError(e)
            })?;

            let _: () = redis_conn
                .set_ex(format!("qotd:{}", language), quote_json, 86400)
                .await
                .map_err(|e| AppError::RedisError(e))?;

            Ok(response_quote)
        }
        Some((_, None)) => Err(AppError::NotFound("No quote content available".to_string())),
        None => Err(AppError::NotFound(
            "No quote found for this language".to_string(),
        )),
    }
}
