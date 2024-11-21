pub mod add_quotes;
pub mod health;

use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(add_quotes::add_quote);
}
