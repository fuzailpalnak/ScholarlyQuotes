use crate::services;
use crate::utils;
use actix_web::{web, Scope};

pub fn qotd_routes() -> Scope {
    actix_web::web::scope("/qotd")
        .service(
            web::resource(utils::languages::Language::English.as_str())
                .route(web::get().to(services::quotes::get_qotd_english)),
        )
        .service(
            web::resource(utils::languages::Language::RomanUrdu.as_str())
                .route(web::get().to(services::quotes::get_qotd_roman_urdu)),
        )
}
