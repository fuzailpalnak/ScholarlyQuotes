pub mod fetch;

use actix_web::{web, Scope};

pub fn quotes_routes() -> Scope {
    actix_web::web::scope("/random_quote")
        .service(web::resource("/english").route(web::get().to(fetch::get_quote_english)))
        .service(web::resource("/arabic").route(web::get().to(fetch::get_quote_arabic)))
        .service(web::resource("/roman_urdu").route(web::get().to(fetch::get_quote_roman_urdu)))
}
