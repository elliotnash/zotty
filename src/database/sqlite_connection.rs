use serenity::async_trait;
use std::fs::File;
use std::sync::Arc;
use std::path::Path;
use path_absolutize::*;
use tokio::sync::Mutex;
use sqlx::{Connection, Executor, sqlite::SqliteConnection as Sqlite};

use super::Database;

#[derive(Debug)]
pub struct SqliteConnection {
    connection: Arc<Mutex<Sqlite>>
}
impl SqliteConnection {
    pub async fn new(path: &str) -> SqliteConnection {
        let db_url = format!("sqlite:{}", get_absolute_path(path));
        println!("Connecting to database: {}", db_url);
        let connection = Sqlite::connect(&db_url).await
            .expect("Failed to connect to sqlite database");
        SqliteConnection {connection: Arc::new(Mutex::new(connection))}
    }
}

#[async_trait]
impl Database for SqliteConnection {
    async fn initialize(&mut self) {
        let mut conn = self.connection.lock().await;
        let qr = conn.execute("BEGIN").await.expect("Failed to query");
        dbg!(qr);
    }
}

fn get_absolute_path(path: &str) -> String {
    let path = Path::new(path);
    let path = path.absolutize().unwrap().to_path_buf();
    if !path.exists() {File::create(&path).expect("Unable to create sqlite database");};
    path.to_str().unwrap().to_string()
}
