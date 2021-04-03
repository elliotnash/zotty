use chrono::{DateTime, Utc};
use serenity::async_trait;

use crate::config::DatabaseType;
use super::CONFIG;

pub mod sqlite_connection;
use sqlite_connection::SqliteConnection;

#[derive(Debug)]
pub struct DBUser {
    pub level: i32,
    pub xp: i32,
    pub last_xp: DateTime<Utc>
}

#[async_trait]
pub trait Database: std::fmt::Debug + Send {
    // returns a User struct with information about recent msgs
    async fn get_user(&mut self, guild_id: String, user_id: String) -> DBUser;
}

pub async fn new_database() -> Box<dyn Database> {
    match CONFIG.get().unwrap().database.db_type {
        DatabaseType::Sqlite => {
            Box::new((SqliteConnection::new(&CONFIG.get().unwrap().database.path)).await)
        }
    }
}
