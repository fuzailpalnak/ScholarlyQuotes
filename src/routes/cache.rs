use crate::helper::oauth;
use crate::models::data::AppState;
use crate::models::errors::AppError;
use crate::utils;
use crate::{helper, models::data::CacheResponse};
use actix_web::{middleware::from_fn, web, HttpResponse, Scope};
use futures_util::future;

pub fn cache_routes() -> Scope {
    actix_web::web::scope("/cache_qotd").service(
        web::resource("/")
            .wrap(from_fn(oauth::admin_check))
            .route(web::post().to(cache_qotd)),
    )
}

async fn cache_qotd(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let tasks = utils::constants::Language::variants()
        .into_iter()
        .map(|lang| {
            helper::quotes::update_qotd_cache_for_language(
                app_state.db.as_ref(),
                app_state.redis_client.as_ref(),
                lang.as_str(),
            )
        })
        .collect::<Vec<_>>();

    future::join_all(tasks).await;

    let value = CacheResponse {
        message: "Operation completed successfully".to_string(),
        success: true,
    };

    Ok(HttpResponse::Ok().json(value))
}
