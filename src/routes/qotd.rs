use crate::helper;
use crate::helper::oauth;
use crate::models::data::AppState;
use crate::models::errors::AppError;
use crate::utils;
use actix_web::{middleware::from_fn, web, HttpResponse, Scope};

pub fn qotd_routes() -> Scope {
    actix_web::web::scope("/qotd")
        .service(
            web::resource(utils::languages::Language::English.as_str())
                .wrap(from_fn(oauth::rate_limit))
                .route(web::get().to(get_qotd_english)),
        )
        .service(
            web::resource(utils::languages::Language::RomanUrdu.as_str())
                .wrap(from_fn(oauth::rate_limit))
                .route(web::get().to(get_qotd_roman_urdu)),
        )
}

async fn get_qotd_english(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db_conn = app_state.db.as_ref();
    let redis_client = app_state.redis_client.as_ref();
    let response = helper::quotes::get_qotd_by_language(
        db_conn,
        redis_client,
        utils::languages::Language::English.as_str(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(response))
}

async fn get_qotd_roman_urdu(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let db_conn = app_state.db.as_ref();
    let redis_client = app_state.redis_client.as_ref();
    let response = helper::quotes::get_qotd_by_language(
        db_conn,
        redis_client,
        utils::languages::Language::RomanUrdu.as_str(),
    )
    .await?;
    Ok(HttpResponse::Ok().json(response))
}
