use actix_web::Scope;

pub mod auth;
pub mod oauth;
pub mod file;

pub fn routes() -> Scope {
    Scope::new("").service(auth::routes()).service(oauth::routes()).service(file::routes())
}
