pub mod series;

use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

fn db_path() -> Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".local/share")))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;
    
    let binge_dir = data_dir.join("binge");
    std::fs::create_dir_all(&binge_dir)?;
    
    Ok(binge_dir.join("binge.db"))
}

pub fn connect() -> Result<Connection> {
    let path = db_path()?;
    let conn = Connection::open(path)?;
    init_tables(&conn)?;
    Ok(conn)
}

fn init_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS series (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            alias TEXT,
            status TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}
