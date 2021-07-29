use std::fs;
use serde::Deserialize;

use crate::HOME_DIR;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub colours: Colours,
    pub database: DatabaseOptions,
    pub modules: Modules,
    pub web: Web
}
#[derive(Debug, Deserialize)]
pub struct Colours {
    pub help: i32,
    pub moderator: i32,
    pub music: i32,
    pub commands: i32,
    pub error: i32,
    pub ranks: i32
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    Sqlite
}
#[derive(Debug, Deserialize)]
pub struct DatabaseOptions {
    pub db_type: DatabaseType
}

#[derive(Debug, Deserialize)]
pub struct Modules {
    pub ranks: Ranks
}

#[derive(Debug, Deserialize)]
pub struct Ranks {
    pub default_level_up_message: String
}

#[derive(Debug, Deserialize)]
pub struct Web {
    pub port: u16,
    pub oauth: OAuth
}

#[derive(Debug, Deserialize)]
pub struct OAuth {
    pub api_url: String,
    pub client_id: String,
    pub client_secret: String
}

impl Config {
    pub fn from_file() -> Self {
        let mut config_path = HOME_DIR.get().unwrap().clone();
        config_path.push("resources");
        config_path.push("config");
        config_path.set_extension("toml");
        let config_file = fs::read_to_string(config_path)
            .expect("Error reading config file");
        toml::from_str(&config_file).unwrap()
    }
}