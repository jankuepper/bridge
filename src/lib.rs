use actix_web::{App, HttpServer, web};
use anyhow::Result;
use sqlx::SqlitePool;
use std::path::Path;

use crate::db::index::migrate;

mod db;
mod handlers;

pub async fn run() -> Result<()> {
    let db_dir = Path::new("./db");
    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir)?;
    }
    const DB_URL: &str = "sqlite:./db/sqlite.sqlite3?mode=rwc";
    let db = SqlitePool::connect(DB_URL).await?;
    migrate(&db).await?;

    let client = reqwest::Client::new();
    let db_ref = web::Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_ref.clone())
            .app_data(web::Data::new(client.clone()))
            .configure(handlers::config)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await?;
    Ok(())
}
