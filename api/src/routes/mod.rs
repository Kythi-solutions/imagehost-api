use actix_web::Scope;

pub mod auth;

pub fn routes() -> Scope {
    Scope::new("").service(auth::routes())
}
