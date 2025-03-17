use crate::db::queries::pg;
use crate::models::data::AppState;
use crate::models::errors::AppError;
use crate::utils;
use actix_web::{web, HttpResponse, Scope};

pub fn quotes_routes() -> Scope {
    actix_web::web::scope("/random_quote")
        .service(
            web::resource(utils::constants::Language::English.as_str())
                .route(web::get().to(get_quote_english)),
        )
        .service(
            web::resource(utils::constants::Language::Arabic.as_str())
                .route(web::get().to(get_quote_arabic)),
        )
        .service(
            web::resource(utils::constants::Language::RomanUrdu.as_str())
                .route(web::get().to(get_quote_roman_urdu)),
        )
}

async fn get_quotes_by_language(
    app_state: web::Data<AppState>,
    language: &str,
) -> Result<HttpResponse, AppError> {
    let db_conn = app_state.db.as_ref();
    let response = pg::fetch_random_quote_by_language(db_conn, language).await?;
    Ok(HttpResponse::Ok().json(response))
}

async fn get_quote_english(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(app_state, utils::constants::Language::English.as_str()).await
}

async fn get_quote_arabic(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(app_state, utils::constants::Language::Arabic.as_str()).await
}

async fn get_quote_roman_urdu(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    get_quotes_by_language(app_state, utils::constants::Language::RomanUrdu.as_str()).await
}
