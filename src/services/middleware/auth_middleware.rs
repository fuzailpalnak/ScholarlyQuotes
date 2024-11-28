use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::{body::MessageBody, middleware::Next};
use log::debug;

use crate::utils;

pub async fn validate_auth_token(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    debug!("Middleware triggered for request: {:?}", req);

    let admin_secret = utils::env_utils::load_admin_secret().unwrap();
    match utils::jwt_utils::extract_bearer_token(&req) {
        Ok(token) => {
            match utils::jwt_utils::is_user_admin_or_premium(token.as_str(), &admin_secret) {
                Ok(_) => next.call(req).await,
                Err(err) => Err(err.into()),
            }
        }
        Err(err) => Err(err.into()),
    }
}
