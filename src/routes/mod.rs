pub mod health;
pub mod quotes;

use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(quotes::quotes_routes());
}
