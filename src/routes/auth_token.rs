use actix_web::{HttpRequest, HttpResponse};

use crate::errors;
use crate::utils;

#[actix_web::get("/get_auth_token")]
pub async fn generate_auth_token(req: HttpRequest) -> Result<HttpResponse, errors::AppError> {
    // Extract the bearer token from the request
    let input_secret = utils::jwt::extract_bearer(req)?;

    // Get the client secret and validate it
    let client_secret = utils::jwt::get_client_secret()?;
    if input_secret != client_secret {
        return Err(errors::AppError::Unauthorized(
            "Unauthorized: Invalid client secret".to_string(),
        ));
    }

    // Get the admin secret and generate the token
    let admin_secret = utils::jwt::get_admin_secret()?;
    let token = utils::jwt::generate_token(&admin_secret)
        .map_err(|err| errors::AppError::Unauthorized(err.to_string()))?;

    Ok(HttpResponse::Ok().body(token))
}
