use crate::errors;
use actix_web::dev::ServiceRequest;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub role: String,
}

/// Generate a token with a specific role and expiration time.
pub fn generate_token_with_role(
    role: &str,
    secret: &str,
    expiry_secs: u64,
) -> Result<String, errors::AppError> {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(expiry_secs))
        .ok_or(errors::AppError::NotFound("NOT FOUND".to_string()))?
        .duration_since(UNIX_EPOCH)
        .map_err(errors::AppError::SystemTimeError)?
        .as_secs() as usize;

    let claims = Claims {
        exp: expiration,
        role: role.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(errors::AppError::TokenError)?;
    Ok(token)
}

/// Decode and validate a JWT token, returning claims if valid.
pub fn decode_and_validate_token(token: &str, secret: &str) -> Result<Claims, errors::AppError> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|err| {
        error!("JWT decode error: {:?}", err);
        errors::AppError::Unauthorized("Invalid token".to_string())
    })?;
    Ok(token_data.claims)
}

/// Extract the Bearer token from the Authorization header.
pub fn extract_bearer_token(req: &ServiceRequest) -> Result<String, errors::AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(errors::AppError::Unauthorized(
            "Missing Authorization header".to_string(),
        ))?
        .to_str()
        .map_err(|_| errors::AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(errors::AppError::Unauthorized(
            "Invalid Authorization format".to_string(),
        ));
    }

    Ok(auth_header["Bearer ".len()..].to_owned())
}
