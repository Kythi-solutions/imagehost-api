use actix_web::{ get, web, HttpRequest, Result };
use oauth2::{ reqwest, AuthorizationCode, CsrfToken };
use serde::{ Deserialize, Serialize };

use crate::infrastructure::{
    http::response::{ DynamicData, JsonResponse },
    service::oauth::OAuthService,
};

#[derive(Deserialize)]
struct OAuthRequest {
    code: String,
    state: String,
}

#[get("/authorize")]
async fn authorize(
    _request: HttpRequest,
    oauth_request: web::Query<OAuthRequest>,
    config: web::Data<configurator::Settings>
) -> Result<JsonResponse, JsonResponse> {
    let csrf_token = CsrfToken::new(oauth_request.state.to_owned());
    let auth_code = AuthorizationCode::new(oauth_request.code.to_owned());

    let client = OAuthService::client(config.discord_oauth.to_owned());

    let http_client = reqwest::ClientBuilder
        ::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        // TODO: change this policy from none
        // TODO: handle client builder error
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let _token_res = client.exchange_code(auth_code.to_owned()).request_async(&http_client).await;

    Ok(JsonResponse::success(Some(DynamicData::String(auth_code.secret().to_string()))))
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    identifier: String,
    password: String,
}
