pub mod create;
pub mod fetch;

use crate::services::middleware::auth_middleware;
use actix_web::middleware::from_fn;
use actix_web::{web, Scope};

pub fn citations_scope() -> Scope {
    actix_web::web::scope("/citations")
        .service(
            web::resource("/create")
                .wrap(from_fn(auth_middleware::validate_auth_token))
                .route(web::post().to(create::create_quote_handler)),
        )
        .service(web::resource("/random").route(web::get().to(fetch::get_random_quote_handler)))
}
