use crate::errors::AppError;
use dotenv::dotenv;
use firebase_rs::Firebase;
use log::info;
use std::env;

pub async fn setup_db() -> Result<Firebase, AppError> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL");

    match db_url {
        Ok(url) => {
            let firebase_conn = match Firebase::new(&url) {
                Ok(conn) => conn,
                Err(err) => {
                    return Err(AppError::DatabaseError(format!(
                        "Failed to connect to Firebase: {}",
                        err
                    )));
                }
            };

            info!("Successfully connected to database.");

            return Ok(firebase_conn);
        }
        Err(err) => return Err(AppError::DatabaseError(err.to_string())),
    };
}
