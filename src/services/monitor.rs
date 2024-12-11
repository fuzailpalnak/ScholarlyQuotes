use crate::services::database;
use crate::{errors::AppError, models::LastUpdate};
use chrono::{Local, NaiveDate};
use firebase_rs::Firebase;
use log::info;
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub async fn monitor_update(db: Arc<Firebase>) -> Result<(), AppError> {
    let mut interval = interval(Duration::from_secs(6 * 60 * 60));

    loop {
        interval.tick().await;

        match is_update_needed(&db).await {
            Ok(true) => {
                if let Err(e) = perform_update(&db, &Local::now().date_naive()).await {
                    log::error!("Error during database update: {:?}", e);
                    return Err(e);
                }
            }
            Ok(false) => {}
            Err(e) => {
                log::error!("Error during database check: {:?}", e);
                return Err(e);
            }
        }
    }
}

async fn is_update_needed(db: &Arc<Firebase>) -> Result<bool, AppError> {
    let today_date = Local::now().date_naive();

    let last_update = database::get_last_update_date(db).await.map_err(|e| {
        AppError::DatabaseError(format!("Failed to fetch the last update date: {}", e))
    })?;

    let last_update_date = parse_last_update(&last_update).map_err(|e| {
        AppError::DatabaseError(format!("Failed to parse the last update date: {}", e))
    })?;

    Ok(last_update_date < today_date)
}

fn parse_last_update(last_update: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(last_update, "%Y-%m-%d")
}

async fn perform_update(db: &Arc<Firebase>, today_date: &NaiveDate) -> Result<(), AppError> {
    let today_date_string = today_date.format("%Y-%m-%d").to_string();

    database::update_last_update_date(
        &db,
        LastUpdate {
            last_update: today_date_string,
        },
    )
    .await
    .map_err(|e| AppError::DatabaseError(format!("Failed to set last update date: {}", e)))?;

    database::update_quote_of_the_day(&db).await.map_err(|e| {
        AppError::DatabaseError(format!("Failed to perform database quote update: {}", e))
    })?;

    info!("Database updated successfully.");
    Ok(())
}
