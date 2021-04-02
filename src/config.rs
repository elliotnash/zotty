use std::fs;
use serde::Deserialize;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_file();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub colours: Colours
}
#[derive(Debug, Deserialize)]
pub struct Colours {
    pub help: i32,
    pub moderator: i32,
    pub music: i32,
    pub commands: i32
}

impl Config {
    fn from_file() -> Self {
        let config_file = fs::read_to_string("./config.toml")
            .expect("Something went wrong reading the file");
        toml::from_str(&config_file).unwrap()
    }
}