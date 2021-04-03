use serenity::async_trait;

use crate::config::DatabaseType;
use super::CONFIG;

pub mod sqlite_connection;
use sqlite_connection::SqliteConnection;

#[async_trait]
pub trait Database: std::fmt::Debug + Send {
    // Make sure database structure is setup
    async fn initialize(&mut self);
}

pub async fn new_database() -> Box<dyn Database> {
    match CONFIG.get().unwrap().database.db_type {
        DatabaseType::Sqlite => {
            Box::new((SqliteConnection::new(&CONFIG.get().unwrap().database.path)).await)
        }
    }
}
