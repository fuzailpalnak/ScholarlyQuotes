use crate::errors;
use dotenv::from_path;
use log::error;
use std::env;

pub fn load_admin_secret() -> Result<String, errors::AppError> {
    load_env_var("TOKEN_SECRET", "Admin secret not configured")
}

pub fn load_admin_api_key() -> Result<String, errors::AppError> {
    load_env_var("ADMIN_API_KEY", "Admin API key not configured")
}

fn load_env_var(key: &str, error_message: &str) -> Result<String, errors::AppError> {
    let env_url = env::var("ENV_PATH").unwrap_or_else(|_| "src/.env".to_string());
    from_path(env_url).map_err(|err| {
        error!("Failed to load .env file: {}", err);
        errors::AppError::NotFound("Failed to load environment variables".to_string())
    })?;

    env::var(key).map_err(|err| {
        error!("{} not found: {}", key, err);
        errors::AppError::NotFound(error_message.to_string())
    })
}
