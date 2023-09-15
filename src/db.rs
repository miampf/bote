use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

use crate::config::get_app_directory;

async fn create_db(connection_string: &str) -> Result<(), anyhow::Error> {
    Sqlite::create_database(connection_string).await?;

    let db = SqlitePool::connect(connection_string).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS packages 
        (
            id INTEGER PRIMARY KEY NOT NULL, 
            pkg_name TEXT,
            pkg_owner TEXT,
            library_name TEXT,
            build_file_location TEXT,
            build_file_hash TEXT,
            own INTEGER
        );",
    )
    .execute(&db)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXIST libraries
        (
            id INTEGER PRIMARY KEY NOT NULL,
            library_name TEXT,
            library_owner TEXT,
            DHT_key TEXT,
            own INTEGER
        );",
    )
    .execute(&db)
    .await?;

    Ok(())
}

/// open_db() opens the sqlite database and creates it if it does not exist.
pub async fn open_db() -> Result<SqlitePool, anyhow::Error> {
    let app_dir = get_app_directory()?;
    let sqlite_connection_string = "sqlite://".to_string() + app_dir.as_str() + "/bote.db";
    let db_exists = Sqlite::database_exists(&sqlite_connection_string).await?;

    if !db_exists {
        create_db(&sqlite_connection_string).await?;
    }

    let db = SqlitePool::connect(&sqlite_connection_string).await?;

    Ok(db)
}
