use mime::Mime;
use serde_derive::{ Deserialize };
use log::trace;

use actix_multipart::form::{ json::Json as MpJson, tempfile::TempFile, MultipartForm };
use actix_web::{ post, web, HttpRequest, Result };

use sea_orm::{ ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter };
use entity::entities::{ prelude::*, sea_orm_active_enums::ProviderEnum };

use crate::infrastructure::{
    http::response::{ DynamicData, JsonResponse },
    repository::access_token::AccessTokenRepository,
};

#[post("/upload")]
async fn upload(
    request: HttpRequest,
    MultipartForm(data): MultipartForm<UploadForm>,
    _database: web::Data<DatabaseConnection>,
    token_repo: web::Data<AccessTokenRepository>
) -> Result<JsonResponse, JsonResponse> {
    let token_header = match request.headers().get("kythi-token") {
        Some(token) =>
            match token.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return Err(
                        JsonResponse::bad_request(
                            DynamicData::String("Invalid token provided.".to_string())
                        )
                    );
                }
            }
        None => {
            return Err(
                JsonResponse::bad_request(
                    DynamicData::String("Invalid token provided.".to_string())
                )
            );
        }
    };

    let find_token = match token_repo.find_by_token(token_header.to_owned()).await {
        Ok(token) =>
            match token {
                Some(token) => token,
                None => {
                    return Err(
                        JsonResponse::bad_request(
                            DynamicData::String("Invalid token provided.".to_string())
                        )
                    );
                }
            }
        Err(err) => {
            trace!("{}", err);
            return Err(JsonResponse::internal_error());
        }
    };

    return Ok(JsonResponse::success(Some(DynamicData::String(token_header))));

    // let user = match
    //     User::Entity::find()
    //         .filter(
    //             Condition::any()
    //                 .add(User::Column::Username.eq(data.identifier.clone()))
    //                 .add(User::Column::Email.eq(data.identifier.clone()))
    //         )
    //         .one(database.as_ref()).await
    // {
    //     Ok(user) =>
    //         match user {
    //             Some(user) => user,
    //             None => {
    //                 return Err(JsonResponse::generic_login_error());
    //             }
    //         }
    //     Err(err) => {
    //         trace!("{}", err);
    //         return Err(JsonResponse::internal_error());
    //     }
    // };

    // let credentials = match cred_repo.by_user_id(user.id).await {
    //     Ok(cred) => {
    //         if cred.is_empty() {
    //             return Err(
    //                 JsonResponse::bad_request(
    //                     DynamicData::String(
    //                         "User doesn't have a password attached, please use the provider you used to signup".to_string()
    //                     )
    //                 )
    //             );
    //         }

    //         cred
    //     }
    //     Err(err) => {
    //         trace!("{}", err);
    //         return Err(JsonResponse::internal_error());
    //     }
    // };

    // let hashed_password = match
    //     credentials.iter().find(|x| x.provider == Some(ProviderEnum::Password))
    // {
    //     Some(pass) =>
    //         match PasswordHash::new(&pass.secret) {
    //             Ok(hash) => hash,
    //             Err(err) => {
    //                 trace!("{}", err);
    //                 return Err(JsonResponse::internal_error());
    //             }
    //         }
    //     None => {
    //         return Err(
    //             JsonResponse::bad_request(
    //                 DynamicData::String(
    //                     "User doesn't have a password attached, please use the provider you used to signup".to_string()
    //                 )
    //             )
    //         );
    //     }
    // };

    // if let Err(_) = validate_hash(data.password.clone(), hashed_password) {
    //     return Err(JsonResponse::generic_login_error());
    // }

    // let _ = Identity::login(&request.extensions(), user.id.to_string());

    Ok(JsonResponse::success(None))
}

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}
