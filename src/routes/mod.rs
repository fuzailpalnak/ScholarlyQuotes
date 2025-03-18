pub mod cache;
pub mod health;
pub mod oauth;
pub mod qotd;
pub mod quotes;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(quotes::quotes_routes())
        .service(qotd::qotd_routes())
        .service(oauth::oauth_routes())
        .service(cache::cache_routes());
}
