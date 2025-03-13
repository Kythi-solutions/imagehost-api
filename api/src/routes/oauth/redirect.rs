use actix_web::{ get, web, HttpRequest, Result };
use log::info;
use oauth2::{ reqwest, AuthorizationCode, CsrfToken, Scope };
use sea_orm::metric::Info;
use serde::{ Deserialize, Serialize };

use crate::infrastructure::{
    http::response::{ DynamicData, JsonResponse },
    service::oauth::OAuthService,
};

// use oauth2::{
//     basic::BasicClient,
//     reqwest,
//     self::{
//         AuthUrl,
//         AuthorizationCode,
//         ClientId,
//         ClientSecret,
//         CsrfToken,
//         RedirectUrl,
//         Scope,
//         TokenResponse,
//         TokenUrl,
//     },
// };

#[derive(Deserialize)]
struct OAuthRequest {
    code: String,
    state: String,
}

#[get("/redirect")]
async fn redirect(
    _request: HttpRequest,
    config: web::Data<configurator::Settings>
) -> Result<JsonResponse, JsonResponse> {
    let client = OAuthService::client(config.discord_oauth.to_owned());

    let (authorize_url, _) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("guilds.join".to_string()))

        .url();

    info!("{}", authorize_url);

    Ok(JsonResponse::success(Some(DynamicData::String(authorize_url.to_string()))))
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    identifier: String,
    password: String,
}
