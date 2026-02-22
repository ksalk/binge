use super::Connection;
use anyhow::Result;
use rusqlite::params;
use uuid::Uuid;

pub struct Series {
    pub id: Uuid,
    pub title: String,
    pub alias: Option<String>,
    pub status: String,
}

impl Series {
    pub fn new(title: String, alias: Option<String>, status: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            alias,
            status,
        }
    }
}

pub fn insert(conn: &Connection, series: &Series) -> Result<()> {
    conn.execute(
        "INSERT INTO series (id, title, alias, status) VALUES (?1, ?2, ?3, ?4)",
        params![
            series.id.to_string(),
            series.title,
            series.alias,
            series.status,
        ],
    )?;
    Ok(())
}

pub fn clear(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM series", [])?;
    Ok(())
}
