use crate::db::queries;
use crate::utils;
use chrono::Utc;
use log::info;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::time::{self, Duration};

pub async fn qotd_scheduler(db_conn: Arc<DatabaseConnection>, redis_client: Arc<redis::Client>) {
    let mut interval = time::interval(Duration::from_secs(86400));
    info!("QOTD Scheduler Started");
    loop {
        interval.tick().await;
        for lang in utils::languages::Language::variants() {
            let last_update: Result<i64, _> =
                queries::get_last_qotd_update_timestamp(redis_client.as_ref(), lang.as_str()).await;

            match last_update {
                Ok(last_timestamp) => {
                    let current_time = Utc::now().timestamp();
                    let elapsed_hours = (current_time - last_timestamp) / 3600;

                    if elapsed_hours >= 24 {
                        if let Err(e) =
                            queries::set_daily_qotd(db_conn.as_ref(), redis_client.as_ref()).await
                        {
                            log::error!("Failed to update QOTD: {:?}", e);
                        }
                    }
                }
                Err(_) => {
                    if let Err(e) =
                        queries::set_daily_qotd(db_conn.as_ref(), redis_client.as_ref()).await
                    {
                        log::error!("Failed to update QOTD: {:?}", e);
                    }
                }
            }
        }
    }
}
