use crate::errors;
use crate::utils;
use actix_web::{web, HttpResponse};
use utils::jwt_utils::Role;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TokenRequest {
    role: Role,
}

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    token: String,
}

pub async fn generate_client_token_handler(
    token_request: web::Json<TokenRequest>,
) -> Result<HttpResponse, errors::AppError> {
    let admin_secret = utils::env_utils::load_admin_secret().unwrap();

    let role = &token_request.role;

    match utils::jwt_utils::generate_token_with_role(role.clone(), &admin_secret) {
        Ok(token) => Ok(HttpResponse::Ok().json(TokenResponse { token })),
        Err(err) => Err(err),
    }
}
