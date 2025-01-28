use std::io::Error;

use actix_web::{services, web::Data, App, HttpServer};

// use biscuit_auth::{KeyPair, PrivateKey};
use database;
use infrastructure::repository::user::UserRepository;
use log::info;

pub mod infrastructure;
pub mod routes;

pub async fn run(config: configurator::Settings) -> Result<(), Error> {
    let server_config = (config.server.ip, config.server.port);

    // let biscuit_private_key = PrivateKey::from_bytes_hex(&config.biscuit.private_key)
    //     .expect("Error: Failed to parse biscuit private key");

    //let biscuit_public_key = KeyPair::from(&biscuit_private_key).public();

    info!("Initializing database pool...");

    let db_conn = database::pool::create_pool(config.database.url)
        .await
        .expect("Error: Failed to connect to database");

    info!("Initializing repositories...");

    let user_repository = UserRepository::new(db_conn.clone());

    info!("Starting server...");

    HttpServer::new(move || {
        App::new()
            //.wrap(BiscuitMiddleware::new(biscuit_public_key)) // apply to routes that need auth
            .app_data(Data::new(db_conn.to_owned()))
            .app_data(Data::new(user_repository.to_owned()))
            .service(services![routes::routes()])
    })
    .bind(server_config)?
    .run()
    .await
}
