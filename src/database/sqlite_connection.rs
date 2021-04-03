use serenity::async_trait;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::path::Path;
use path_absolutize::*;
use tokio::sync::Mutex;
use sqlx::{Connection, Executor, sqlite::SqliteConnection as Sqlite};

use super::Database;

pub struct SqliteConnection {
    connection: Arc<Mutex<Sqlite>>
}
impl SqliteConnection {
    pub async fn new(path: &str) -> SqliteConnection {
        let connection = Sqlite::connect(&format!("sqlite:{}", get_absolute_path(path)) ).await
            .expect("Failed to connect to sqlite database");
        let mut connection = SqliteConnection {connection: Arc::new(Mutex::new(connection))};
        connection.initialize().await;
        println!("We just initialized the db");
        connection
    }
}
#[async_trait]
impl Database for SqliteConnection {
    async fn initialize(&mut self) {
        println!("initalize called");
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
