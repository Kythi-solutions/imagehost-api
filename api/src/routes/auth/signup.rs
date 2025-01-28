use actix_web::{
    http::StatusCode, post, web, HttpResponse, HttpResponseBuilder, Responder, ResponseError,
};
use derive_more::derive::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::infrastructure::{
    common::validator::{validate_email, validate_username},
    repository::{self, user::UserRepository, Repository},
};

#[derive(Serialize, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Display, Error, Serialize, Deserialize)]
enum SignupError {
    #[display("internal error")]
    InternalError,

    #[display("bad request")]
    BadClientData {
        username_error: Option<String>,
        email_error: Option<String>,
        password_error: Option<String>,
    },

    #[display("timeout")]
    Timeout,
}

#[post("/signup")]
async fn signup(
    data: web::Json<SignupRequest>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, SignupError> {
    if let Err(err) = validate_username(data.username.clone()) {
        return Err(SignupError::BadClientData {
            username_error: Some(err.as_string()),
            email_error: None,
            password_error: None,
        });
    }

    if let Err(err) = validate_email(data.email.clone()) {
        return Err(SignupError::BadClientData {
            username_error: None,
            email_error: Some(err.to_string()),
            password_error: None,
        });
    }

    Ok(user_repo.by_id(1).await.unwrap().unwrap().id.to_string())
}

impl ResponseError for SignupError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(self)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadClientData { .. } => StatusCode::BAD_REQUEST,
            SignupError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            SignupError::Timeout => StatusCode::REQUEST_TIMEOUT,
        }
    }
}
