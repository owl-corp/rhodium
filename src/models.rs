use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<HashMap<String, Value>>,
}

#[derive(Deserialize)]
pub struct HashRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct HashResponse {
    pub hex: String,
    pub i64: i64,
}
