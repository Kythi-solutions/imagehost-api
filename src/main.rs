use actix_web::web::trace;
use api;
use log::debug;

mod logging;

#[actix_web::main]
async fn main() {
    let config = configurator::Settings::new().expect("Error: Unable to populate config structs");

    logging::setup_logger(config.clone()).expect("Error: Failed to setup logger");

    debug!("Failelele");

    api::run(config).await.expect("Error: Failed to start api")
}
