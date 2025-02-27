use std::io::Error;

use actix_identity::IdentityMiddleware;
use actix_session::SessionMiddleware;
use actix_web::{
    cookie::Key,
    http::StatusCode,
    services,
    web::{self, Data},
    App, HttpServer,
};

use database;
use infrastructure::{
    http::response::JsonResponse,
    repository::{credential::CredentialRepository, user::UserRepository},
};
use log::{info, trace};

pub mod infrastructure;
pub mod routes;

pub async fn run(config: configurator::Settings) -> Result<(), Error> {
    let server_config = (config.server.ip, config.server.port);

    info!("Initializing database pool...");

    let db_conn = database::pool::create_pool(config.database.url)
        .await
        .expect("Error: Failed to connect to database");

    let redis_store = actix_session::storage::RedisSessionStore::new(config.redis.url)
        .await
        .expect("Error: Failed to connect to redis");

    info!("Initializing repositories...");

    let user_repository = UserRepository::new(db_conn.clone());
    let credential_repository = CredentialRepository::new(db_conn.clone());

    info!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                Key::from(config.identity.secret.clone().as_bytes()),
            ))
            .app_data(Data::new(db_conn.to_owned()))
            .app_data(Data::new(user_repository.to_owned()))
            .app_data(Data::new(credential_repository.to_owned()))
            .app_data(web::JsonConfig::default().error_handler(|_, req| {
                trace!("{:?}", req);
                actix_web::Error::from(JsonResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    None,
                    None,
                ))
            }))
            .service(services![routes::routes()])
    })
    .bind(server_config)?
    .run()
    .await
}
