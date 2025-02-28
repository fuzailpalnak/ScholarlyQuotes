use crate::utils;

use log::info;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;

use sea_orm::Database;

pub async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    let db_url =
        utils::env::load_env_var("DATABASE_URL").map_err(|err| DbErr::Custom(err.to_string()))?;
    info!("{}", db_url);

    let db = Database::connect(&db_url).await?;
    info!("Successfully Connected to DB");
    Ok(db)
}
