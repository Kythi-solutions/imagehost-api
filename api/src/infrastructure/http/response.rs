use std::fmt::Display;

use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DynamicData {
    String(String),
    JsonValue(serde_json::Value),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonResponse {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<DynamicData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DynamicData>,
}

impl Display for JsonResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "code: {}", self.code)
    }
}

impl JsonResponse {
    pub fn new(code: u16, error: Option<DynamicData>, data: Option<DynamicData>) -> Self {
        Self { code, error, data }
    }

    pub fn bad_request(error: DynamicData) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST.as_u16(),
            data: None,
            error: Some(error),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            data: None,
            error: Some(DynamicData::String("Internal Error".to_string())),
        }
    }

    pub fn generic_login_error() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST.as_u16(),
            data: None,
            error: Some(DynamicData::String(
                "Some data you provided is incorrect".to_string(),
            )),
        }
    }

    pub fn success(data: Option<DynamicData>) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: data,
            error: None,
        }
    }
}

impl From<JsonResponse> for HttpResponse {
    fn from(response: JsonResponse) -> Self {
        HttpResponse::build(StatusCode::from_u16(response.code).unwrap()).json(response)
    }
}

impl Responder for JsonResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::from(self)
    }
}

impl ResponseError for JsonResponse {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).unwrap()
    }
}
