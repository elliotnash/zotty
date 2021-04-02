use serenity::async_trait;
use diesel::prelude::*;
use diesel::sqlite;
use std::sync::{Arc, Mutex};
use diesel::sqlite::SqliteConnection as Sqlite;

use super::Database;

pub struct SqliteConnection {
    connection: Arc<Mutex<Sqlite>>
}
impl SqliteConnection {
    pub fn new(path: &str) -> SqliteConnection {
        let connection = Sqlite::establish(path);
        SqliteConnection {connection: Arc::new(Mutex::new(connection.unwrap()))}
    }
}
#[async_trait]
impl Database for SqliteConnection {
    async fn connect(&self) {
        println!("connecting here");
    }
}
