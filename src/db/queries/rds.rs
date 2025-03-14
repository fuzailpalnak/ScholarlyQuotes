use crate::models::data::ResponseQuote;
use crate::models::errors::AppError;
use log::info;

use chrono::{Datelike, NaiveDate, TimeZone, Utc};
use chrono_tz::Europe::Berlin;

use redis::AsyncCommands;

use serde_json::Error as SerdeError;

pub async fn update_qotd_into_redis(
    redis: &redis::Client,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let mut conn = redis
        .get_async_connection()
        .await
        .map_err(|err| AppError::RedisError(err))?;

    update_qotd_in_redis(&mut conn, quote).await?;
    update_qotd_reset_time_in_redis(&mut conn, quote).await?;

    Ok(())
}

pub async fn update_qotd_in_redis(
    conn: &mut redis::aio::Connection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let key = format!("qotd:{}", quote.language);
    info!("Caching QOTD");

    let quote_json =
        serde_json::to_string(quote).map_err(|err: SerdeError| AppError::SerdeError(err))?;

    let _: () = conn
        .set(key, quote_json)
        .await
        .map_err(|err| AppError::RedisError(err))?;
    Ok(())
}

pub async fn update_qotd_reset_time_in_redis(
    conn: &mut redis::aio::Connection,
    quote: &ResponseQuote,
) -> Result<(), AppError> {
    let now_utc = Utc::now();
    let now_eu = now_utc.with_timezone(&Berlin);
    let tomorrow: NaiveDate = now_eu
        .date_naive()
        .succ_opt()
        .ok_or_else(|| AppError::NotFound("Failed to get the next day's date".to_string()))?;

    let midnight_eu = Berlin
        .with_ymd_and_hms(tomorrow.year(), tomorrow.month(), tomorrow.day(), 0, 0, 0)
        .single()
        .ok_or_else(|| AppError::NotFound("Failed to create midnight time.".to_string()))?;

    let key = format!("qotd:reset_countdown:{}", quote.language);

    let _: () = conn.set(key, midnight_eu.timestamp()).await?;
    Ok(())
}

pub async fn get_last_qotd_update_timestamp(
    redis_client: &redis::Client,
    lang: &str,
) -> Result<i64, AppError> {
    let mut conn = redis_client.get_async_connection().await?;
    let key = format!("qotd:reset_countdown:{}", lang);
    let time_till_reset: Result<i64, _> = conn.get(key).await;
    info!("Seconds till Reset: {:?}", time_till_reset);

    match time_till_reset {
        Ok(timestamp) => Ok(timestamp),
        Err(e) => Err(AppError::RedisError(e)),
    }
}
