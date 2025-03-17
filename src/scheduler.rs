use crate::helper;
use crate::utils;

use futures_util::future;
use log::info;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::time::{self, Duration};

pub async fn qotd_cache_update_scheduler(
    db_conn: Arc<DatabaseConnection>,
    redis_client: Arc<redis::Client>,
) {
    let mut interval = time::interval(Duration::from_secs(86400));
    info!("QOTD Scheduler Started");

    loop {
        interval.tick().await;

        let tasks = utils::constants::Language::variants()
            .into_iter()
            .map(|lang| {
                helper::quotes::update_qotd_cache_for_language(
                    db_conn.clone(),
                    redis_client.clone(),
                    lang.as_str(),
                )
            })
            .collect::<Vec<_>>();

        future::join_all(tasks).await;
    }
}
