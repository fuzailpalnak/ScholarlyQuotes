use crate::errors;
use actix_web::dev::ServiceRequest;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use log::error;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,   // level 0
    Premium, // level 1
    Regular, // level 2
    Free,    // level 3
}

impl Role {
    /// Returns the default expiration time in seconds for the role
    pub fn default_expiration(&self) -> usize {
        match self {
            Role::Admin => usize::MAX,
            Role::Premium => 365 * 24 * 60 * 60,
            Role::Regular => 30 * 24 * 60 * 60,
            Role::Free => 60 * 60,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub role: Role,
}

/// Generate a token with a specific role and expiration time.
pub fn generate_token_with_role(role: Role, secret: &str) -> Result<String, errors::AppError> {
    let current_time = current_unix_timestamp()?;
    let expiration_time = calculate_expiration(&role, current_time);

    let claims = Claims {
        exp: expiration_time,
        role,
    };

    encode_token(&claims, secret)
}

/// Retrieve the current Unix timestamp as `usize`.
fn current_unix_timestamp() -> Result<usize, errors::AppError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(errors::AppError::SystemTimeError)
        .map(|duration| duration.as_secs() as usize)
}

/// Calculate the expiration time based on the role and current time.
fn calculate_expiration(role: &Role, current_time: usize) -> usize {
    match role {
        Role::Admin => usize::MAX,
        _ => current_time + role.default_expiration(),
    }
}

/// Encode a JWT token with the given claims and secret.
fn encode_token(claims: &Claims, secret: &str) -> Result<String, errors::AppError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(errors::AppError::TokenError)
}

/// Decode and validate a JWT token, returning claims if valid.
pub fn is_admin(token: &str, secret: &str) -> Result<Claims, errors::AppError> {
    let token_data = decode_token(token, secret)?;
    let role = &token_data.claims.role;
    match role {
        Role::Admin => Ok(token_data.claims),
        _ => {
            error!("Insufficient role: {:?}", role);
            Err(errors::AppError::Unauthorized(
                "Insufficient role".to_string(),
            ))
        }
    }
}

/// Decode a JWT token and return the token data.
fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, errors::AppError> {
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

    Ok(token_data)
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
