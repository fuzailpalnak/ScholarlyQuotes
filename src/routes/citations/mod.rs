pub mod create;
pub mod fetch;

use crate::services::middleware::authentication;
use actix_web::middleware::from_fn;
use actix_web::{web, Scope};

pub fn citations_scope() -> Scope {
    actix_web::web::scope("/citations")
        .service(
            web::resource("/create")
                .wrap(from_fn(authentication::validate_auth_token))
                .route(web::post().to(create::create_quote_handler)),
        )
        .service(
            web::resource("/random")
                .wrap(from_fn(authentication::validate_auth_token))
                .route(web::get().to(fetch::get_random_quote_handler)),
        )
        .service(
            web::resource("/quote_of_the_day")
                .route(web::get().to(fetch::quote_of_the_day_handler)),
        )
}
