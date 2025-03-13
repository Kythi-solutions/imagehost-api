use config::{ Config, ConfigError, File };
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Logger {
    pub level: i32,
    pub output_file: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Identity {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct DiscordOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub auth_url: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub redis: Redis,
    pub logger: Logger,
    pub identity: Identity,
    pub discord_oauth: DiscordOAuth,
}

fn find_config(dir: &str) -> Result<Config, ConfigError> {
    let root_dir = std::env
        ::current_dir()
        .expect("Error: Failed to get current dir (configurator)");

    let binding = root_dir.join(dir);
    let config_dir = binding.to_str().expect("Error: Failed to parse config dir");

    return Config::builder().add_source(File::with_name(config_dir)).build();
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_search = find_config("../Config");

        match config_search {
            Ok(config) => {
                return config.try_deserialize();
            }
            Err(_) => {
                let config_search = find_config("Config").expect("Error: Failed to find config");

                return config_search.try_deserialize();
            }
        }
    }
}

pub trait OAuthTrait {
    fn client_id(&self) -> String;
    fn client_secret(&self) -> String;
    fn token_url(&self) -> String;
    fn auth_url(&self) -> String;
}

impl OAuthTrait for DiscordOAuth {
    fn client_id(&self) -> String {
        self.client_id.to_owned()
    }
    fn client_secret(&self) -> String {
        self.client_secret.to_owned()
    }
    fn token_url(&self) -> String {
        self.token_url.to_owned()
    }
    fn auth_url(&self) -> String {
        self.auth_url.to_owned()
    }
}
