use chrono::{DateTime, Utc};
use serenity::async_trait;
use std::time::Duration;
use std::{fs::File, time::UNIX_EPOCH};
use std::sync::Arc;
use std::path::Path;
use path_absolutize::*;
use tokio::sync::Mutex;
use rusqlite::Connection;

use super::{Database, DBUser};

#[derive(Debug)]
pub struct SqliteConnection {
    connection: Arc<Mutex<Connection>>
}
impl SqliteConnection {
    pub async fn new(path: &str) -> SqliteConnection {
        let db_path = get_absolute_path(path);
        println!("Connecting to database: {}", db_path);
        let connection = Connection::open(&db_path)
            .expect("Failed to connect to sqlite database");
        SqliteConnection {connection: Arc::new(Mutex::new(connection))}
    }
}

fn get_absolute_path(path: &str) -> String {
    let path = Path::new(path);
    let path = path.absolutize().unwrap().to_path_buf();
    if !path.exists() {File::create(&path).expect("Unable to create sqlite database");};
    path.to_str().unwrap().to_string()
}

#[async_trait]
impl Database for SqliteConnection {
    async fn get_user(&mut self, guild_id: String, user_id: String) -> DBUser {

        let conn = self.connection.lock().await;

        //create table in db if it doesn't exist for this server
        conn.execute(&format!("
        CREATE TABLE IF NOT EXISTS '{}' (
            user_id INTEGER PRIMARY KEY,
            level INTEGER NOT NULL DEFAULT 0,
            xp INTEGER NOT NULL DEFAULT 0,
            last_xp INTEGER NOT NULL DEFAULT 0
        );
        ", guild_id), []).expect("Failed to create tables");

        conn.execute(&format!("
        INSERT OR IGNORE INTO '{0}' values ({1}, 0, 0, 0);
        ", guild_id, user_id), [])
            .expect("Failed to insert ____ into user");

        let mut query = conn.prepare(&format!("
        SELECT level, xp, last_xp FROM '{0}' WHERE user_id = {1};
        ", guild_id, user_id)).expect("Failed to query database");

        let mut db_user_iter = query.query_map([], |row| {
            //let levels: i32 = row.get("levels").unwrap();
            //dbg!(&levels);
            let duration = UNIX_EPOCH + Duration::from_secs(row.get("last_xp").unwrap());
            let datetime = DateTime::<Utc>::from(duration);
            Ok(DBUser {
                level: row.get("level").unwrap(),
                xp: row.get("xp").unwrap(),
                last_xp: datetime
            })
        }).expect("Failed to query database");

        db_user_iter.next().unwrap().unwrap()

    }

    async fn get_rank(&mut self, guild_id: String, db_user: &DBUser) -> i32 {

        let conn = self.connection.lock().await;

        let mut query = conn.prepare(&format!("
        SELECT COUNT() FROM '{0}'
	        WHERE level > {1} OR 
		        (level = {1} AND xp> {2});
        ", guild_id, db_user.level, db_user.xp)).expect("Failed to query database");
        let test: i32 = query.query_row([], |row| {
            Ok(row.get(0).unwrap())
        }).unwrap();
        test+1
    }

    async fn set_user_xp(&mut self, guild_id: String, user_id: String, xp: i32) {

        let conn = self.connection.lock().await;

        conn.execute(&format!("
        UPDATE '{0}' SET xp = {2}, last_xp = {3} WHERE user_id = {1};
        ", guild_id, user_id, xp, Utc::now().timestamp()), [])
            .expect("Failed to update user");

    }

    async fn set_user_level(&mut self, guild_id: String, user_id: String, level: i32, xp: i32) {

        let conn = self.connection.lock().await;

        conn.execute(&format!("
        UPDATE '{0}' SET level = {2}, xp = {3}, last_xp = {4} WHERE user_id = {1};
        ", guild_id, user_id, level, xp, Utc::now().timestamp()), [])
            .expect("Failed to update user");

    }
}
