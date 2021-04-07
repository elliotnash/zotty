use chrono::{DateTime, Utc};
use r2d2_sqlite::{SqliteConnectionManager, rusqlite::params};
use r2d2::Pool;
use serenity::{async_trait, model::id::UserId};
use std::time::Duration;
use std::{fs::File, time::UNIX_EPOCH};
use std::path::Path;
use path_absolutize::*;
use tracing::info;

use super::{Database, DBUser};

#[derive(Debug)]
pub struct SqliteConnection {
    pool: Pool<SqliteConnectionManager>
}
impl SqliteConnection {
    pub async fn new(path: &str) -> SqliteConnection {
        let db_path = get_absolute_path(path);
        info!("Connecting to database: {}", db_path);
        let manager = SqliteConnectionManager::file(&db_path);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .unwrap();
        SqliteConnection {pool}
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

        let pool = self.pool.clone();
        let conn = pool.get().expect("Failed to get sqlite connection");
        drop(pool);

        //create table in db if it doesn't exist for this server
        conn.execute(&format!("
        CREATE TABLE IF NOT EXISTS '{}_levels' (
            user_id INTEGER PRIMARY KEY,
            level INTEGER NOT NULL DEFAULT 0,
            xp INTEGER NOT NULL DEFAULT 0,
            last_xp INTEGER NOT NULL DEFAULT 0
        );
        ", guild_id), params![]).expect("Failed to create tables");

        conn.execute(&format!("
        INSERT OR IGNORE INTO '{0}_levels' values ({1}, 0, 0, 0);
        ", guild_id, user_id), params![])
            .expect("Failed to insert ____ into user");

        let mut query = conn.prepare(&format!("
        SELECT user_id, level, xp, last_xp FROM '{0}_levels' WHERE user_id = {1};
        ", guild_id, user_id)).expect("Failed to query database");

        query.query_row(params![], |row| {
            let duration: i64 = row.get("last_xp").unwrap();
            let duration = UNIX_EPOCH + Duration::from_secs(duration as u64);
            let datetime = DateTime::<Utc>::from(duration);
            let user_id: i64 = row.get("user_id").unwrap();
            Ok(DBUser {
                user_id: user_id.to_string(),
                level: row.get("level").unwrap(),
                xp: row.get("xp").unwrap(),
                last_xp: datetime
            })
        }).expect("Failed to query database")

    }

    async fn get_rank(&mut self, guild_id: String, db_user: &DBUser) -> i32 {

        let pool = self.pool.clone();
        let conn = pool.get().expect("Failed to get sqlite connection");

        let mut query = conn.prepare(&format!("
        SELECT COUNT() FROM '{0}_levels'
	        WHERE level > {1} OR 
		        (level = {1} AND xp> {2});
        ", guild_id, db_user.level, db_user.xp)).expect("Failed to query database");
        let rank_index: i32 = query.query_row(params![], |row| {
            Ok(row.get(0).unwrap())
        }).unwrap();
        rank_index+1
    }

    async fn set_user_xp(&mut self, guild_id: String, user_id: String, xp: i32) {

        let pool = self.pool.clone();
        let conn = pool.get().expect("Failed to get sqlite connection");

        conn.execute(&format!("
        UPDATE '{0}_levels' SET xp = {2}, last_xp = {3} WHERE user_id = {1};
        ", guild_id, user_id, xp, Utc::now().timestamp()), params![])
            .expect("Failed to update user");

    }

    async fn set_user_level(&mut self, guild_id: String, user_id: String, level: i32, xp: i32) {

        let pool = self.pool.clone();
        let conn = pool.get().expect("Failed to get sqlite connection");

        conn.execute(&format!("
        UPDATE '{0}_levels' SET level = {2}, xp = {3}, last_xp = {4} WHERE user_id = {1};
        ", guild_id, user_id, level, xp, Utc::now().timestamp()), params![])
            .expect("Failed to update user");

    }
}
