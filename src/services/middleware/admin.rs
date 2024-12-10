use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::{body::MessageBody, middleware::Next};

use crate::{errors, utils};

pub async fn check_admin_api_key(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    match req.headers().get("api_key") {
        Some(api_key) => match utils::env::load_admin_api_key() {
            Ok(admin_api_key) => {
                if api_key.to_str().unwrap() == admin_api_key.to_string() {
                    next.call(req).await
                } else {
                    Err(
                        errors::AppError::Unauthorized("Invalid or missing API key".to_string())
                            .into(),
                    )
                }
            }
            Err(err) => Err(err.into()),
        },
        None => {
            Err(errors::AppError::Unauthorized("Invalid or missing API key".to_string()).into())
        }
    }
}
