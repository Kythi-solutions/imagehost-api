use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

fn find_config(dir: &str) -> Result<Config, ConfigError> {
    let root_dir =
        std::env::current_dir().expect("Error: Failed to get current dir (configurator)");

    let binding = root_dir.join(dir);
    let config_dir = binding.to_str().expect("Error: Failed to parse config dir");

    return Config::builder()
        .add_source(File::with_name(config_dir))
        .build();
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_search = find_config("../Config");

        match config_search {
            Ok(config) => return config.try_deserialize(),
            Err(_) => {
                let config_search = find_config("Config").expect("Error: Failed to find config");

                return config_search.try_deserialize();
            }
        }
    }
}
