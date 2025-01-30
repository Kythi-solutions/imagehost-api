use actix_identity::Identity;
use actix_web::{
    post,
    web::{self},
    HttpMessage, HttpRequest, Result,
};
use entity::entities::{prelude::*, sea_orm_active_enums::ProviderEnum};
use log::trace;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::infrastructure::{
    common::{
        hashing,
        validator::{validate_email, validate_password, validate_username},
    },
    http::response::{DynamicData, JsonResponse},
};

#[derive(Serialize, Deserialize)]
struct SignupRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignupError {
    username_error: Option<String>,
    email_error: Option<String>,
    password_error: Option<String>,
}

#[post("/signup")]
async fn signup(
    request: HttpRequest,
    data: web::Json<SignupRequest>,
    database: web::Data<DatabaseConnection>,
) -> Result<JsonResponse, JsonResponse> {
    if let Err(err) = validate_username(data.username.clone()) {
        return Err(JsonResponse::bad_request(DynamicData::JsonValue(
            SignupError {
                email_error: None,
                password_error: None,
                username_error: Some(err.to_string()),
            }
            .into(),
        )));
    }

    if let Err(err) = validate_email(data.email.clone()) {
        return Err(JsonResponse::bad_request(DynamicData::JsonValue(
            SignupError {
                email_error: Some(err.to_string()),
                password_error: None,
                username_error: None,
            }
            .into(),
        )));
    }

    if let Err(err) = validate_password(data.password.clone()) {
        return Err(JsonResponse::bad_request(DynamicData::JsonValue(
            SignupError {
                email_error: None,
                password_error: Some(err.to_string()),
                username_error: None,
            }
            .into(),
        )));
    }

    match User::Entity::find()
        .filter(
            Condition::any()
                .add(User::Column::Username.eq(data.username.clone()))
                .add(User::Column::Email.eq(data.email.clone())),
        )
        .one(database.as_ref())
        .await
    {
        Ok(user) => match user {
            Some(_) => {
                return Err(JsonResponse::bad_request(DynamicData::String(
                    "Username/Email is already in use".to_string(),
                )));
            }
            None => {}
        },
        Err(err) => {
            trace!("{}", err);
            return Err(JsonResponse::internal_error());
        }
    };

    let hashed_password = match hashing::hash_password(data.password.clone()) {
        Ok(hash) => hash,
        Err(err) => {
            trace!("{}", format!("Error: Failed to hash password {}", err));
            return Err(JsonResponse::internal_error());
        }
    };

    let user_am = match database
        .transaction::<_, User::Model, DbErr>(|txn| {
            Box::pin(async move {
                let user_am = User::ActiveModel {
                    username: Set(data.username.clone()),
                    email: Set(data.email.clone()),
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
            return Err(JsonResponse::internal_error());
        }
    };

    let _ = Identity::login(&request.extensions(), user_am.id.to_string());

    Ok(JsonResponse::success(None))
}

impl Into<Value> for SignupError {
    fn into(self) -> Value {
        json!(self)
    }
}
