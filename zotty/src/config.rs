use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub colours: Colours,
    pub database: DatabaseOptions,
    pub modules: Modules
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
    pub db_type: DatabaseType,
    pub path: String
}

#[derive(Debug, Deserialize)]
pub struct Modules {
    pub ranks: Ranks
}

#[derive(Debug, Deserialize)]
pub struct Ranks {
    pub default_level_up_message: String
}

impl Config {
    pub fn from_file() -> Self {
        let config_file = fs::read_to_string("./config.toml")
            .expect("Something went wrong reading the file");
        toml::from_str(&config_file).unwrap()
    }
}