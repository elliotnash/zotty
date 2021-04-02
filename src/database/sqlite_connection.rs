use serenity::async_trait;

use super::Database;

pub struct SqliteConnection {
    pub path: String
}
#[async_trait]
impl Database for SqliteConnection {
    async fn connect(&self) {
        println!("connecting here");
    }
}
