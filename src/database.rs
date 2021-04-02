use std::sync::Mutex;
use serenity::async_trait;
use lazy_static::lazy_static;

use crate::config::DatabaseType;
use super::CONFIG;

pub mod sqlite_connection;
use sqlite_connection::SqliteConnection;

lazy_static! {
    pub static ref DATABASE: Mutex<Box<dyn Database>> = Mutex::new(new_database());
}

#[async_trait]
pub trait Database: Send {
    async fn connect(&self);
}

pub fn new_database() -> Box<dyn Database> {
    match CONFIG.database.db_type {
        DatabaseType::Sqlite => {
            Box::new(SqliteConnection {path: CONFIG.database.path.clone()})
        }
    }
}
