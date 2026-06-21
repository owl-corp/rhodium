use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(serialize_with = "serialize_extra")]
    pub extra: Option<HashMap<String, Value>>,
}

#[allow(clippy::ref_option)]
fn serialize_extra<S>(
    extra: &Option<HashMap<String, Value>>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match extra {
        Some(map) => map.serialize(serializer),
        None => HashMap::<String, Value>::new().serialize(serializer),
    }
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
