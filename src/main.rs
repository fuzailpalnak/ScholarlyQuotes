mod db_conn;
mod entities;
mod errors;
mod routes;
mod services;
mod utils;

use crate::errors::AppError;
use actix_web::{web, App, HttpServer};
// use log::error;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    // Set up the database connection
    let data_persistence = db_conn::setup_db().await;

    // Handle the result of the database connection setup
    match data_persistence {
        Ok(db) => {
            let db = Arc::new(db);
            // Start the HTTP server
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(db.clone())) // Pass db connection to the app
                    .configure(routes::config_routes) // Configure routes
            })
            .bind("0.0.0.0:8080")? // Bind to address
            .workers(1) // Number of worker threads
            .run() // Run the server
            .await?;

            // Return Ok after the server runs successfully
            Ok(())
        }
        Err(e) => Err(AppError::DatabaseError(e)),
    }
}
