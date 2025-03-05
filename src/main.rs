mod db;
mod entities;
mod errors;
mod routes;
mod services;
mod utils;

use crate::errors::AppError;
use actix_web::{web, App, HttpServer};
use log::info;
use std::sync::Arc;

use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis_client: Arc<RedisClient>,
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    // Set up the database connection
    let db = db::conn::setup_db()
        .await
        .map_err(AppError::DatabaseError)?;
    let redis_client = db::conn::steup_redis().await?;

    let db = Arc::new(db);
    let redis_client = Arc::new(redis_client);

    info!("Starting Actix-web server on 0.0.0.0:8080...");

    actix_rt::spawn(services::scheduler::qotd_scheduler(
        db.clone(),
        redis_client.clone(),
    ));

    let app_state = web::Data::new(AppState {
        db: db,
        redis_client: redis_client,
    });
    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Pass shared app state
            .configure(routes::config_routes) // Configure routes
    })
    .bind("0.0.0.0:8080")? // Bind to the specified address
    .workers(4) // Use 4 worker threads for better performance
    .run()
    .await?;

    Ok(())
}
