use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub application_host: String,
    pub database_settings: DatabaseSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let settings = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{run_mode}")))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        println!("debug: {:?}", settings);

        settings.try_deserialize()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.hostname, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.hostname, self.port
        )
    }
}
