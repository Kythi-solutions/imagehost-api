use actix_web::Scope;

pub mod login;
pub mod signup;

pub fn routes() -> Scope {
    Scope::new("/auth").service(login::login).service(signup::signup)
}
