use serde::{Deserialize, Serialize};

use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use unkey::Client as UnkeyClient;

#[derive(Clone)]
pub struct UnkeyApiId(pub String);

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub redis_client: Arc<RedisClient>,
    pub unkey_client: UnkeyClient,
    pub unkey_api_id: UnkeyApiId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub id: i32,
    pub content: String,
    pub author: String,
    pub reference: String,
    pub language: String,
}

#[derive(Deserialize, Serialize)]
pub struct KeyRequest {
    pub owner_id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct KeyResponse {
    pub key: String,
    pub key_id: String,
}

#[derive(Serialize)]
pub struct CacheResponse {
    pub message: String,
    pub success: bool,
}
