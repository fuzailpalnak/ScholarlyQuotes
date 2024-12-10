pub mod citations;
pub mod health;
pub mod tokens;

use crate::services::middleware::admin;

use actix_web::{middleware::from_fn, web};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(citations::citations_scope())
        .service(
            web::resource("/generate_client_token")
                .wrap(from_fn(admin::check_admin_api_key))
                .route(web::post().to(tokens::generate_client_token_handler)),
        );
}
