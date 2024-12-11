mod errors;
mod models;
mod routes;
mod services;
mod utils;

use crate::errors::AppError;
use crate::services::monitor;
use actix_web::{web, App, HttpServer};

// use log::error;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    // Handle the result of the database connection setup
    match services::database::db_conn().await {
        Ok(db) => {
            // let db = Arc::new(db);
            let db = Arc::new(db);

            // Spawn the background task safely
            actix_rt::spawn(monitor::monitor_update(db.clone()));
            
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
        Err(e) => Err(e),
    }
}
