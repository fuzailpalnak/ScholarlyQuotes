use crate::models::data::{AppState, UnkeyApiId};
use crate::models::errors::AppError::{self, ApiKeyError};

use crate::utils;
use actix_web::body::MessageBody;
use actix_web::http::header::HeaderMap;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::{Error, ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
    web,
};

use log::{error, info};

use serde_json::Value;
use unkey::models::{VerifyKeyRequest, VerifyKeyResponse};
use unkey::Client as UnkeyClient;

impl From<UnkeyApiId> for String {
    fn from(api_id: UnkeyApiId) -> Self {
        api_id.0
    }
}

pub async fn connect_to_oauth_server() -> Result<(UnkeyClient, UnkeyApiId), Error> {
    let unkey_root_key = utils::env::load_env_var("UNKEY_ROOT_KEY")
        .map_err(|_| actix_web::error::ErrorInternalServerError("UNKEY_ROOT_KEY must be set"))?;

    let unkey_api_id = UnkeyApiId(
        utils::env::load_env_var("UNKEY_API_ID")
            .map_err(|_| actix_web::error::ErrorInternalServerError("UNKEY_API_ID must be set"))?,
    );

    let unkey_client = UnkeyClient::new(&unkey_root_key);
    Ok((unkey_client, unkey_api_id))
}

pub async fn owner_check(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    info!("Owner Middleware start");

    let data = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorInternalServerError("AppState missing"))?;

    let authorization_header = extract_authorization_header(req.headers())?;

    let response = verify_api_key(&data, &authorization_header)
        .await
        .map_err(|err| {
            error!("API key verification error: {}", err);
            ErrorUnauthorized("API key verification failed")
        })?;

    if !response.valid {
        return Err(ErrorUnauthorized("Invalid API key"));
    }

    if let Ok(false) = is_admin(&response.meta) {
        return Err(ErrorUnauthorized("Not Authorized to generate new keys"));
    }

    next.call(req).await
}

pub async fn rate_limit(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    info!("Rate Limit Middleware start");

    let data = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorInternalServerError("AppState missing"))?;

    let authorization_header = extract_authorization_header(req.headers())?;

    let response = verify_api_key(&data, &authorization_header)
        .await
        .map_err(|err| {
            error!("API key verification error: {}", err);
            ErrorUnauthorized("API key verification failed")
        })?;

    if let Ok(true) = is_admin(&response.meta) {
        return Err(ErrorUnauthorized("Incorrect Use of API Key"));
    }

    match check_rate_limit(response).await {
        Ok(true) => next.call(req).await,
        Ok(false) => Err(ErrorUnauthorized(
            "Rate limit exceeded for the day. Please try again later.",
        )),
        Err(err) => {
            error!("Rate limit check error: {}", err);
            Err(ErrorUnauthorized(err.to_string()))
        }
    }
}

async fn check_rate_limit(response: VerifyKeyResponse) -> Result<bool, AppError> {
    if let Some(ratelimit) = response.ratelimit {
        info!("Remining Rate Limit: {}", ratelimit.remaining);
        info!("Response Valid: {}", response.valid);
        if ratelimit.remaining == 0 || !response.valid {
            return Err(ApiKeyError(
                "Rate limit exceeded. Please try again later.".to_string(),
            ));
        }
    }

    Ok(true)
}

fn extract_authorization_header(headers: &HeaderMap) -> Result<String, Error> {
    headers
        .get("Authorization")
        .and_then(|val| val.to_str().ok())
        .filter(|val| val.starts_with("Bearer "))
        .map(|val| val.trim_start_matches("Bearer ").to_string())
        .ok_or_else(|| ErrorUnauthorized("Invalid or missing Authorization header"))
}

async fn verify_api_key(
    data: &web::Data<AppState>,
    key: &str,
) -> Result<VerifyKeyResponse, AppError> {
    let verify_request = VerifyKeyRequest {
        key: key.to_string(),
        api_id: data.unkey_api_id.clone().into(),
    };

    match data.unkey_client.verify_key(verify_request).await {
        Ok(response) => Ok(response),

        Err(err) => {
            error!("Key verification request failed: {:?}", err);
            Err(ApiKeyError(
                "Key verification failed. Please try again later.".to_string(),
            ))
        }
    }
}

fn is_admin(meta: &Option<Value>) -> Result<bool, AppError> {
    let is_admin = meta
        .as_ref() // Convert &Option<Value> to Option<&Value>
        .and_then(|meta| meta.as_object())
        .and_then(|meta| meta.get("admin"))
        .and_then(|admin_value| admin_value.as_bool())
        .unwrap_or(false);
    info!("Is Admin: {}", is_admin);
    Ok(is_admin)
}
