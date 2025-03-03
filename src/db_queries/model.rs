use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseQuote {
    pub id: i32,
    pub content: String,
    pub author: String,
    pub reference: String,
    pub language: String,
}
