use std::io::Error;

use actix_web::{web::Data, App, HttpServer};

use biscuit_auth::{KeyPair, PrivateKey};
use database;
use infrastructure::middleware::biscuit::BiscuitMiddleware;
use log::info;

pub mod infrastructure;
pub mod routes;

pub async fn run(config: configurator::Settings) -> Result<(), Error> {
    let server_config = (config.server.ip, config.server.port);

    let biscuit_private_key = PrivateKey::from_bytes_hex(&config.biscuit.private_key)
        .expect("Error: Failed to parse biscuit private key");

    let biscuit_public_key = KeyPair::from(&biscuit_private_key).public();

    info!("Initializing database pool...");

    let db_conn = database::pool::create_pool(config.database.url)
        .await
        .expect("Error: Failed to connect to database");

    info!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .wrap(BiscuitMiddleware::new(biscuit_public_key))
            .app_data(Data::new(db_conn.clone()))
        // TODO: services here...
    })
    .bind(server_config)?
    .run()
    .await
}
