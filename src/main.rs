mod db;
mod entities;
mod helper;
mod models;
mod routes;
mod utils;

use crate::helper::oauth::connect_to_oauth_server;
use crate::models::data::AppState;
use crate::models::errors::AppError;

use actix_web::{middleware::Logger, web, App, HttpServer};
use reqwest::Client;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db = Arc::new(db::conn::setup_db().await?);
    let redis_client = Arc::new(db::conn::steup_redis().await?);

    let (unkey_client, unkey_api_id) = connect_to_oauth_server().await?;

    let app_state = web::Data::new(AppState {
        db,
        redis_client,
        unkey_client,
        unkey_api_id,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .app_data(web::Data::new(Client::new()).clone())
            .wrap(helper::governor::create_governor(
                &helper::governor::create_governor_config(),
            ))
            .wrap(Logger::default())
            .configure(routes::config_routes)
    })
    .bind("0.0.0.0:8080")?
    .workers(4)
    .run()
    .await?;

    Ok(())
}
