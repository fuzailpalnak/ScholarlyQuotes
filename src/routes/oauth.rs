use crate::helper::oauth;
use crate::models::data::AppState;
use crate::models::data::KeyRequest;
use crate::models::data::KeyResponse;
use crate::models::errors::AppError;
use crate::utils;

use actix_web::{middleware::from_fn, web, HttpResponse, Scope};
use log::error;
use serde_json::to_value;

use unkey::models::ApiKey;
use unkey::models::{
    CreateKeyRequest, CreateKeyResponse, ListKeysRequest, Ratelimit, RatelimitType, Refill,
    RefillInterval, RevokeKeyRequest,
};

use unkey::Client;

pub fn oauth_routes() -> Scope {
    actix_web::web::scope("/generate_key").service(
        web::resource("new")
            .wrap(from_fn(oauth::owner_check))
            .route(web::post().to(generate_key)),
    )
}

async fn generate_key(
    app_state: web::Data<AppState>,
    req_body: web::Json<KeyRequest>,
) -> Result<HttpResponse, AppError> {
    let client = app_state.unkey_client.clone();

    if let Some(existing_key) = find_existing_key(&client, &req_body, &app_state).await? {
        revoke_key(existing_key, &client).await?;
    }

    create_new_key(&client, &req_body, &app_state).await
}

async fn find_existing_key(
    client: &Client,
    req_body: &KeyRequest,
    app_state: &AppState,
) -> Result<Option<ApiKey>, AppError> {
    let list_req = ListKeysRequest {
        api_id: app_state.unkey_api_id.0.clone(),
        owner_id: Some(req_body.owner_id.clone()),
        limit: Some(1),
        cursor: None,
    };

    match client.list_keys(list_req).await {
        Ok(res) => {
            for key in &res.keys {
                println!("{:?}", key); // Print each key
            }
            Ok(res.keys.into_iter().last()) // Return the last key (most recent)
        }
        Err(err) => {
            error!("{:?}", err);
            Err(AppError::ApiKeyError("Failed API LookUp".to_string()))
        }
    }
}

async fn revoke_key(api_key: ApiKey, client: &Client) -> Result<(), AppError> {
    let req = RevokeKeyRequest::new(api_key.id.clone());

    match client.revoke_key(req).await {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{:?}", err);
            Err(AppError::ApiKeyError("Key Revocation Failed".to_string()))
        }
    }
}

async fn create_new_key(
    client: &Client,
    req_body: &KeyRequest,
    app_state: &AppState,
) -> Result<HttpResponse, AppError> {
    let ratelimit = Ratelimit::new(
        RatelimitType::Fast,
        utils::constants::APILimit::RefillRate.as_usize(),
        utils::constants::APILimit::RefillInterval.as_usize(),
        utils::constants::APILimit::TotalRequest.as_usize(),
    );
    let refill = Refill::new(
        utils::constants::APILimit::RefillRate.as_usize(),
        RefillInterval::Daily,
    );
    let meta = serialize_meta(req_body)?;

    let req = CreateKeyRequest::new(app_state.unkey_api_id.clone())
        .set_prefix("qotd")
        .set_remaining(utils::constants::APILimit::TotalRequest.as_usize())
        .set_name(&req_body.name)
        .set_owner_id(&req_body.owner_id)
        .set_ratelimit(ratelimit)
        .set_refill(refill)
        .set_meta(meta);

    match client.create_key(req).await {
        Ok(res) => Ok(build_key_response_from_creation(&res)),
        Err(err) => {
            error!("{:?}", err);
            Err(AppError::ApiKeyError("Key Creation Fail".to_string()))
        }
    }
}

fn build_key_response_from_creation(res: &CreateKeyResponse) -> HttpResponse {
    let response = KeyResponse {
        key: res.key.clone(),
        key_id: res.key_id.clone(),
    };
    HttpResponse::Ok().json(response)
}

fn serialize_meta(req_body: &KeyRequest) -> Result<serde_json::Value, AppError> {
    to_value(req_body)
        .map_err(|e| AppError::ApiKeyError(format!("Failed to serialize meta: {}", e)))
}
