use actix_identity::Identity;
use actix_web::{
    http::StatusCode, post, web, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder,
    Responder, ResponseError,
};
use derive_more::derive::{Display, Error, From};
use entity::entities::{prelude::*, sea_orm_active_enums::ProviderEnum};
use log::trace;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::infrastructure::{
    common::{
        hashing,
        validator::{validate_email, validate_username},
    },
    http::response::JsonResponse,
};

#[derive(Serialize, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Display, Error, Serialize, Deserialize, From)]
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
    request: HttpRequest,
    data: web::Json<SignupRequest>,
    database: web::Data<DatabaseConnection>,
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

    let hashed_password = match hashing::hash_password(data.password.clone()) {
        Ok(hash) => hash,
        Err(err) => {
            trace!("{}", format!("Error: Failed to hash password {}", err));
            return Err(SignupError::InternalError);
        }
    };

    let user_am = match database
        .transaction::<_, User::Model, DbErr>(|txn| {
            Box::pin(async move {
                let user_am = User::ActiveModel {
                    username: Set(data.username.clone()),
                    two_factor: Set(false),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Credential::ActiveModel {
                    user_id: Set(user_am.id),
                    provider: Set(Some(ProviderEnum::Password)),
                    secret: Set(hashed_password),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(user_am)
            })
        })
        .await
    {
        Ok(user_am) => user_am,
        Err(err) => {
            trace!("{:?}", err);
            return Err(SignupError::InternalError);
        }
    };

    let _ = Identity::login(&request.extensions(), user_am.id.to_string());

    Ok("")
}

impl ResponseError for SignupError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).json(JsonResponse {
            code: self.status_code().as_u16(),
            data: None,
            error: Some(self.to_string()),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadClientData { .. } => StatusCode::BAD_REQUEST,
            SignupError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            SignupError::Timeout => StatusCode::REQUEST_TIMEOUT,
        }
    }
}
