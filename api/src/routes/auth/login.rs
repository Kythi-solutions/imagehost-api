use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    identifier: String,
    password: String,
}

#[post("/login")]
pub async fn login(data: web::Json<LoginRequest>) -> impl Responder {
    data.identifier.clone()
}
