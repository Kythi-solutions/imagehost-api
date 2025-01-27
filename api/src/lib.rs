use std::io::Error;

use actix_web::{web::Data, App, HttpServer};

use database;
use log::info;

pub mod infrastructure;
pub mod routes;

pub async fn run(config: configurator::Settings) -> Result<(), Error> {
    let server_config = (config.server.ip, config.server.port);

    let db_conn = database::pool::create_pool(config.database.url)
        .await
        .expect("Error: Failed to connect to database");

    info!("Starting server...");

    HttpServer::new(move || {
        App::new().app_data(Data::new(db_conn.clone()))
        // TODO: services here...
    })
    .bind(server_config)?
    .run()
    .await
}
