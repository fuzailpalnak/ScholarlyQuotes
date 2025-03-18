use crate::db::queries::pg;
use crate::db::queries::rds;
use crate::entities;
use crate::models::data;
use crate::models::data::ResponseQuote;
use crate::models::errors::AppError;
use crate::utils;

use chrono::Utc;
use futures_util::future;
use log::{error, info};
use redis::AsyncCommands;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

async fn update_qotd(db_conn: &DatabaseConnection, redis: &redis::Client) -> Result<(), AppError> {
    let tasks: Vec<_> = utils::constants::Language::variants()
        .into_iter()
        .map(|lang| {
            let db_conn = db_conn.clone();
            let redis = redis.clone();
            async move {
                let response = pg::fetch_random_quote_by_language(&db_conn, lang.as_str()).await?;
                pg::update_qotd_in_db(&db_conn, &response).await?;
                rds::update_qotd_into_redis(&redis, &response).await?;
                Ok::<(), AppError>(())
            }
        })
        .collect();

    future::try_join_all(tasks).await.map(|_| ())
}

fn should_update_cache(last_timestamp: i64) -> bool {
    Utc::now().timestamp() >= last_timestamp
}

async fn update_qotd_cache(
    db_conn: &DatabaseConnection,
    redis_client: &redis::Client,
    lang: &str,
) -> Result<(), AppError> {
    update_qotd(db_conn, redis_client)
        .await
        .map(|_| {
            info!("Successfully updated QOTD for '{}'", lang);
        })
        .map_err(|e| {
            error!("Failed to update QOTD for '{}': {:?}", lang, e);
            e
        })
}

async fn fallback_cache_update(
    db_conn: &DatabaseConnection,
    redis_client: &redis::Client,
    lang: &str,
) -> Result<(), AppError> {
    let mut redis_conn = redis_client.get_async_connection().await.map_err(|e| {
        error!("Failed to get Redis connection: {:?}", e);
        AppError::RedisError(e)
    })?;

    get_qotd_from_db_and_create_redis_cache(db_conn, &mut redis_conn, lang)
        .await
        .map(|_| {
            info!("QOTD for '{}' successfully cached to Redis", lang);
        })
        .map_err(|e| {
            error!("Failed to fetch and cache QOTD for '{}': {:?}", lang, e);
            e
        })
}

pub async fn update_qotd_cache_for_language(
    db_conn: &DatabaseConnection,
    redis_client: &redis::Client,
    lang: &str,
) -> Result<(), AppError> {
    match rds::fetch_last_qotd_timestamp(redis_client, lang).await {
        Ok(last_timestamp) => {
            if should_update_cache(last_timestamp) {
                update_qotd_cache(db_conn, redis_client, lang).await?;
            }
            Ok(())
        }
        Err(e) => {
            fallback_cache_update(db_conn, redis_client, lang).await?;
            error!(
                "Failed to fetch last QOTD timestamp for '{}': {:?}",
                lang, e
            );
            Err(e)
        }
    }
}

pub async fn get_qotd_by_language(
    db_conn: &DatabaseConnection,
    redis_conn: &redis::Client,
    language: &str,
) -> Result<data::ResponseQuote, AppError> {
    let mut redis_conn = redis_conn.get_async_connection().await?;
    let key = format!("qotd:{}", language);

    match redis_conn.get::<_, String>(key.clone()).await {
        Ok(cached_quote) => match serde_json::from_str::<data::ResponseQuote>(&cached_quote) {
            Ok(quote) => {
                log::info!(
                    "Successfully fetched quote from Redis for language: {}",
                    language
                );
                Ok(quote)
            }
            Err(e) => {
                log::error!("Failed to deserialize quote from Redis: {}", e);
                get_qotd_from_db_and_create_redis_cache(db_conn, &mut redis_conn, language).await
            }
        },
        Err(e) => {
            log::error!("Error fetching quote from Redis: {}", e);
            get_qotd_from_db_and_create_redis_cache(db_conn, &mut redis_conn, language).await
        }
    }
}

pub async fn get_qotd_from_db_and_create_redis_cache(
    db_conn: &DatabaseConnection,
    redis_conn: &mut redis::aio::Connection,
    language: &str,
) -> Result<ResponseQuote, AppError> {
    let response_quote = pg::get_qotd_from_db(db_conn, language).await?;

    rds::update_qotd_in_redis(redis_conn, &response_quote).await?;
    rds::update_qotd_reset_time_in_redis(redis_conn, &response_quote).await?;

    Ok(response_quote)
}
