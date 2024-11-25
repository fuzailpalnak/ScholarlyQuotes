pub mod create;
pub mod fetch;

use crate::services::middleware::auth_middleware;
use actix_web::middleware::from_fn;
use actix_web::Scope;

pub fn citations_scope() -> Scope {
    actix_web::web::scope("/citations")
        .route(
            "/create",
            actix_web::web::post()
                .wrap(from_fn(auth_middleware::validate_auth_token))
                .to(create::create_quote_handler),
        )
        .route(
            "/random",
            actix_web::web::get().to(fetch::get_random_quote_handler),
        )
}
