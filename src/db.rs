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
    let connection_string = "sqlite://".to_string() + app_dir.as_str() + "/bote.db";
    let db_exists = Sqlite::database_exists(&connection_string).await?;

    if !db_exists {
        create_db(&connection_string).await?;
    }

    let db = SqlitePool::connect(&connection_string).await?;

    Ok(db)
}

/// write_table_to_db() inserts a new library into a database.
pub async fn write_table_to_db(
    db: &SqlitePool,
    library_name: &str,
    library_owner: &str,
    dht_key: &str,
    own: bool,
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO libraries
        (library_name, library_owner, DHT_key, own)
        VALUES (?, ?, ?, ?);",
    )
    .bind(library_name)
    .bind(library_owner)
    .bind(dht_key)
    .bind(own)
    .execute(db)
    .await?;

    Ok(())
}

/// write_package_to_db() inserts a new package into a database.
pub async fn write_package_to_db(
    db: &SqlitePool,
    pkg_name: &str,
    pkg_owner: &str,
    library_name: &str,
    build_file_location: &str,
    build_file_hash: &str,
    own: bool,
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO packages
        (pkg_name, pkg_owner, library_name, build_file_location, build_file_hash, own)
        VALUES (?, ?, ?, ?, ?, ?);",
    )
    .bind(pkg_name)
    .bind(pkg_owner)
    .bind(library_name)
    .bind(build_file_location)
    .bind(build_file_hash)
    .bind(own)
    .execute(db)
    .await?;

    Ok(())
}
