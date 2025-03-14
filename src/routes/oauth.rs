use crate::helper::oauth;
use crate::models::data::AppState;
use crate::models::data::KeyRequest;
use crate::models::data::KeyResponse;
use crate::models::errors::AppError;
use crate::utils;
use actix_web::{middleware::from_fn, web, HttpResponse, Scope};
use serde_json::to_value;
use unkey::models::CreateKeyRequest;
use unkey::models::Ratelimit;
use unkey::models::RatelimitType;
use unkey::models::Refill;
use unkey::models::RefillInterval;

pub fn oauth_routes() -> Scope {
    actix_web::web::scope("/generate_key")
        .service(
            web::resource(utils::languages::Language::English.as_str())
                .wrap(from_fn(oauth::owner_check))
                .route(web::post().to(generate_key)),
        )
        .service(
            web::resource(utils::languages::Language::RomanUrdu.as_str())
                .wrap(from_fn(oauth::owner_check))
                .route(web::post().to(generate_key)),
        )
}

async fn generate_key(
    app_state: web::Data<AppState>,
    req_body: web::Json<KeyRequest>,
) -> Result<HttpResponse, AppError> {
    let c = app_state.unkey_client.clone();

    let ratelimit = Ratelimit::new(RatelimitType::Fast, 3, 86400000, 3);
    let refill = Refill::new(3, RefillInterval::Daily);
    let meta = to_value(&req_body)
        .map_err(|e| AppError::ApiKeyError(format!("Failed to serialize meta: {}", e)))?;

    let req = CreateKeyRequest::new(app_state.unkey_api_id.clone())
        .set_prefix("qotd")
        .set_remaining(3)
        .set_name(&req_body.name)
        .set_owner_id(&req_body.owner_id)
        .set_ratelimit(ratelimit)
        .set_refill(refill)
        .set_meta(meta);

    match c.create_key(req).await {
        Ok(res) => {
            let response = KeyResponse {
                key: res.key,
                key_id: res.key_id,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => {
            eprintln!("{:?}", err);
            Err(AppError::ApiKeyError("Key Creation Fail".to_string()))
        }
    }
}
