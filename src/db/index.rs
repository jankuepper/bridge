// use std::path::Path;

use anyhow::Result;
use sqlx::{Pool, Sqlite, query};
// use tokio::fs::{read_dir, read_to_string};

pub async fn migrate(db: &Pool<Sqlite>) -> Result<()> {
    // let migration_path = Path::new("");
    // let mut migration_folder_entries = read_dir(migration_path).await?;
    //
    // while let Some(entry) = migration_folder_entries.next_entry().await? {
    //     let _ = query(read_to_string(entry.path()))
    //         .execute(db)
    //         .await
    //         .unwrap();
    // }

    let _ = query(
        "CREATE TABLE IF NOT EXISTS gql_to_rest (
                    id INTEGER PRIMARY KEY NOT NULL, 
                    name TEXT NOT NULL,
                    method TEXT NOT NULL,
                    from_url TEXT NOT NULL,
                    to_url TEXT NOT NULL,
                    query TEXT NOT NULL,
                    version INTEGER NOT NULL
                );",
    )
    .execute(db)
    .await
    .unwrap();

    Ok(())
}
