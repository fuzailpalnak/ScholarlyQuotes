use crate::errors;
use actix_web::HttpRequest;
use dotenv::from_path;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::error;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub role: String,
}

pub fn generate_token(secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(3600))
        .ok_or("Failed to calculate expiration time")?
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;

    let claims = Claims {
        exp: expiration,
        role: "client".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(token)
}

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, errors::AppError> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|err| {
        // Map the decoding error into your custom AppError
        error!("JWT decode error: {:?}", err);
        errors::AppError::Unauthorized("Invalid token".to_string())
    })?;
    Ok(token_data.claims)
}

pub fn extract_bearer(req: HttpRequest) -> Result<String, errors::AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(errors::AppError::Unauthorized(
            "Missing Authorization header".to_string(),
        ))?
        .to_str()
        .map_err(|_| errors::AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    // Ensure the token is in the "Bearer <token>" format
    if !auth_header.starts_with("Bearer ") {
        return Err(errors::AppError::Unauthorized(
            "Invalid Authorization format".to_string(),
        ));
    }

    Ok(auth_header["Bearer ".len()..].to_owned())
}

pub fn get_client_secret() -> Result<String, errors::AppError> {
    let dotenv_path = Path::new("src/.env");

    // Load the .env file from src/ directory
    from_path(dotenv_path).map_err(|err| {
        error!("Failed to load .env file: {}", err);
        errors::AppError::NotFound("Failed to load environment variables".to_string())
    })?;

    let client_secret = env::var("CLIENT_SECRET").map_err(|err| {
        error!("CLIENT_SECRET not found: {}", err);
        errors::AppError::NotFound("CLIENT secret not configured".to_string())
    })?;

    Ok(client_secret)
}

pub fn get_admin_secret() -> Result<String, errors::AppError> {
    let dotenv_path = Path::new("src/.env");

    // Load the .env file from src/ directory
    from_path(dotenv_path).map_err(|err| {
        error!("Failed to load .env file: {}", err);
        errors::AppError::NotFound("Failed to load environment variables".to_string())
    })?;

    let token_secret = env::var("TOKEN_SECRET").map_err(|err| {
        error!("TOKEN_SECRET not found: {}", err);
        errors::AppError::NotFound("Token secret not configured".to_string())
    })?;

    Ok(token_secret)
}
