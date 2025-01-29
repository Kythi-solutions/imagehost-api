use std::fmt::Display;

use actix_web::ResponseError;
use derive_more::derive::{From, Into};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, From, Into)]
pub struct JsonResponse {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl Display for JsonResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code: {}", self.code)
    }
}

impl JsonResponse {
    pub fn new(code: u16, error: Option<String>, data: Option<serde_json::Value>) -> Self {
        Self { code, error, data }
    }
}

impl ResponseError for JsonResponse {}
