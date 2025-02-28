use crate::errors;
use dotenv::from_path;
use log::error;
use std::env;

pub fn load_env_var(key: &str) -> Result<String, errors::AppError> {
    let env_url = env::var("ENV_PATH").unwrap_or_else(|_| ".env".to_string());
    from_path(env_url).map_err(|err| {
        error!("Failed to load .env file: {}", err);
        errors::AppError::NotFound("Failed to load environment variables".to_string())
    })?;

    env::var(key).map_err(|err| {
        error!("{} not found: {}", key, err);
        errors::AppError::NotFound(err.to_string())
    })
}
