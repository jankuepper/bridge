use std::path::Path;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;

pub async fn run(){
    let db_dir = Path::new("/app/db");
    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir)?;
    }
    const DB_URL: &str = "sqlite:///app/db/sqlite.sqlite3";
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("created"),
            Err(err) => panic!("error: {}?", err),
        }
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
}