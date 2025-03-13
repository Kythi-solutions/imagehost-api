use actix_web::Scope;

pub mod authorize;
pub mod redirect;

pub fn routes() -> Scope {
    Scope::new("/oauth").service(authorize::authorize).service(redirect::redirect)
}
