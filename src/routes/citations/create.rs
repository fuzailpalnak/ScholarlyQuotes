use crate::errors;
use crate::models::Quote;
use crate::services::database;

use actix_web::{web, HttpResponse};
use firebase_rs::Firebase;
use log::debug;

use std::sync::Arc;

pub async fn create_quote_handler(
    db: web::Data<Arc<Firebase>>,
    data: web::Json<Quote>,
) -> Result<HttpResponse, errors::AppError> {
    debug!("Input data: {:?}", data);

    database::add(&db, data.into_inner())
        .await
        .map(|quote| HttpResponse::Ok().json(quote))
        .map_err(|err| err)
}
