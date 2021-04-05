use chrono::{DateTime, Utc};
use serenity::async_trait;

use crate::config::DatabaseType;
use super::CONFIG;

pub mod sqlite_connection;
use sqlite_connection::SqliteConnection;

#[derive(Debug, Clone)]
pub struct DBUser {
    pub level: i32,
    pub xp: i32,
    pub last_xp: DateTime<Utc>
}

#[async_trait]
pub trait Database: std::fmt::Debug + Send {
    // returns a User struct with information about recent msgs
    async fn get_user(&mut self, guild_id: String, user_id: String) -> DBUser;
    // returns a users rank
    async fn get_rank(&mut self, guild_id: String, db_user: &DBUser) -> i32;
    // sets a users xp
    async fn set_user_xp(&mut self, guild_id: String, user_id: String, xp: i32);
    // sets a users xp and level
    async fn set_user_level(&mut self, guild_id: String, user_id: String, level: i32, xp: i32);
}

pub async fn new_database() -> Box<dyn Database> {
    match CONFIG.get().unwrap().database.db_type {
        DatabaseType::Sqlite => {
            Box::new((SqliteConnection::new(&CONFIG.get().unwrap().database.path)).await)
        }
    }
}
