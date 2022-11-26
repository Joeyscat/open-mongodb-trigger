use std::{fs, path::Path};

use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub db: DbConfig,
    pub server: ServerConfig,
    pub watcher: WatcherConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DbConfig {
    pub mongodb_uri: String,
    pub db: String,
    pub trigger_coll: String,
    pub function_coll: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WatcherConfig {
    pub scan_delay_seconds: u32,
}

impl Config {
    pub fn load(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let config = fs::read_to_string(filename.as_ref())
            .map_err(|e| Error::ConfigReadError(e.to_string()))?;
        toml::from_str(&config).map_err(|e| Error::ConfigParseError(e.to_string()))
    }
}

impl ServerConfig {
    pub fn url(&self, https: bool) -> String {
        if https {
            format!("https://{}:{}", self.host, self.port)
        } else {
            format!("http://{}:{}", self.host, self.port)
        }
    }
}
