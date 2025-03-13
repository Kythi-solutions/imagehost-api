use actix_identity::Identity;
use actix_web::{ post, web, HttpMessage, HttpRequest, Result };
use argon2::PasswordHash;
use entity::entities::{ prelude::*, sea_orm_active_enums::ProviderEnum };
use log::trace;
use sea_orm::{ ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter };
use serde::{ Deserialize, Serialize };

use crate::infrastructure::{
    common::hashing::validate_hash,
    http::response::JsonResponse,
    repository::credential::CredentialRepository,
};

#[post("/login")]
async fn login(
    request: HttpRequest,
    data: web::Json<LoginRequest>,
    database: web::Data<DatabaseConnection>,
    cred_repo: web::Data<CredentialRepository>
) -> Result<JsonResponse, JsonResponse> {
    let user = match
        User::Entity::find()
            .filter(
                Condition::any()
                    .add(User::Column::Username.eq(data.identifier.clone()))
                    .add(User::Column::Email.eq(data.identifier.clone()))
            )
            .one(database.as_ref()).await
    {
        Ok(user) =>
            match user {
                Some(user) => user,
                None => {
                    return Err(JsonResponse::generic_login_error());
                }
            }
        Err(err) => {
            trace!("{}", err);
            return Err(JsonResponse::internal_error());
        }
    };

    let credentials = match cred_repo.by_user_id(user.id).await {
        Ok(cred) => {
            if cred.is_empty() {
                return Err(JsonResponse::generic_login_error());
            }

            cred
        }
        Err(err) => {
            trace!("{}", err);
            return Err(JsonResponse::internal_error());
        }
    };

    let hashed_password = match
        credentials.iter().find(|x| x.provider == Some(ProviderEnum::Password))
    {
        Some(pass) =>
            match PasswordHash::new(&pass.secret) {
                Ok(hash) => hash,
                Err(err) => {
                    trace!("{}", err);
                    return Err(JsonResponse::internal_error());
                }
            }
        None => {
            return Err(JsonResponse::generic_login_error());
        }
    };

    if let Err(_) = validate_hash(data.password.clone(), hashed_password) {
        return Err(JsonResponse::generic_login_error());
    }

    let _ = Identity::login(&request.extensions(), user.id.to_string());

    Ok(JsonResponse::success(None))
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    identifier: String,
    password: String,
}
