use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    identifier: String,
    password: String
}

#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    req.identifier.clone()
}