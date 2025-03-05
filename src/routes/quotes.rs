use crate::services;
use crate::utils;
use actix_web::{web, Scope};

pub fn quotes_routes() -> Scope {
    actix_web::web::scope("/random_quote")
        .service(
            web::resource(utils::languages::Language::English.as_str())
                .route(web::get().to(services::quotes::get_quote_english)),
        )
        .service(
            web::resource(utils::languages::Language::Arabic.as_str())
                .route(web::get().to(services::quotes::get_quote_arabic)),
        )
        .service(
            web::resource(utils::languages::Language::RomanUrdu.as_str())
                .route(web::get().to(services::quotes::get_quote_roman_urdu)),
        )
}
