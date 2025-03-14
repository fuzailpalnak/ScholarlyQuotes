use crate::models::errors::AppError;
use crate::utils;

use log::info;
use redis::Client;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;

use sea_orm::Database;

pub async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    let db_url =
        utils::env::load_env_var("DATABASE_URL").map_err(|err| DbErr::Custom(err.to_string()))?;
    let db = Database::connect(&db_url).await?;
    info!("Successfully Connected to DB");
    Ok(db)
}

pub async fn steup_redis() -> Result<Client, AppError> {
    let redis_url = utils::env::load_env_var("REDIS_URL")?;
    let redis_client = redis::Client::open(redis_url)?;
    info!("Successfully Connected to Redis");

    Ok(redis_client)
}
