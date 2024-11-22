pub mod auth_token;
pub mod create_citations;
pub mod get_citations;
pub mod health;

use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(create_citations::add_quote)
        .service(get_citations::get_quote);
}
