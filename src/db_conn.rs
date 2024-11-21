use dotenv::dotenv;
use log::info;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use std::env;

use sea_orm::Database;

pub async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://target/scholarlyQuotes.db".to_string());

    let db = Database::connect(&db_url).await?;
    info!("Successfully Connected to DB");
    Ok(db)
}
