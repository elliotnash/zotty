use std::fs;
use serde::Deserialize;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_file();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub prefix: String
}

impl Config {
    fn from_file() -> Self {
        let config_file = fs::read_to_string("./config.toml")
            .expect("Something went wrong reading the file");
        toml::from_str(&config_file).unwrap()
    }
}