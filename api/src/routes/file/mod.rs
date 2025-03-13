use actix_web::Scope;

pub mod upload;

pub fn routes() -> Scope {
    Scope::new("/file").service(upload::upload)
}
