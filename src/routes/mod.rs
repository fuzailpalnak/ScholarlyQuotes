pub mod health;
pub mod quotes;
use crate::utils;
use actix_web::{web, Scope};

pub fn quotes_routes() -> Scope {
    actix_web::web::scope("/random_quote")
        .service(
            web::resource(utils::languages::Language::English.as_str())
                .route(web::get().to(quotes::get_quote_english)),
        )
        .service(
            web::resource(utils::languages::Language::Arabic.as_str())
                .route(web::get().to(quotes::get_quote_arabic)),
        )
        .service(
            web::resource(utils::languages::Language::RomanUrdu.as_str())
                .route(web::get().to(quotes::get_quote_roman_urdu)),
        )
}

pub fn qotd_routes() -> Scope {
    actix_web::web::scope("/qotd")
        .service(
            web::resource(utils::languages::Language::English.as_str())
                .route(web::get().to(quotes::get_qotd_english)),
        )
        .service(
            web::resource(utils::languages::Language::RomanUrdu.as_str())
                .route(web::get().to(quotes::get_qotd_roman_urdu)),
        )
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(quotes_routes())
        .service(qotd_routes());
}
