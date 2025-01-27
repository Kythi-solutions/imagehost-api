use api;

mod logging;

#[actix_web::main]
async fn main() {
    let config = configurator::Settings::new().expect("Error: Unable to populate config structs");

    logging::setup_logger(config.clone()).expect("Error: Failed to setup logger");

    api::run(config).await.expect("Error: Failed to start api")
}
