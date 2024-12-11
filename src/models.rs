use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastUpdate {
    pub last_update: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub content: String,
    pub author: String,
    pub category: Option<String>,
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub content: String,
    pub author: String,
    #[serde(default = "default_category")]
    pub category: Option<String>,
    pub reference: String,
}

pub fn default_category() -> Option<String> {
    Some("Uncategorized".to_string())
}
