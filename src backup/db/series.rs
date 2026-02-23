use super::Connection;
use anyhow::Result;
use rusqlite::params;
use uuid::Uuid;

pub struct Series {
    pub id: Uuid,
    pub title: String,
    pub alias: Option<String>,
    pub status: String,
    pub episodes_schedule: Option<String>,
}

impl Series {
    pub fn new(title: String, alias: Option<String>, status: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            alias,
            status,
            episodes_schedule: None,
        }
    }
}

pub fn insert(conn: &Connection, series: &Series) -> Result<()> {
    conn.execute(
        "INSERT INTO series (id, title, alias, status, episodes_schedule) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            series.id.to_string(),
            series.title,
            series.alias,
            series.status,
            series.episodes_schedule,
        ],
    )?;
    Ok(())
}

pub fn clear(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM series", [])?;
    Ok(())
}

pub fn get_all(conn: &Connection) -> Result<Vec<Series>> {
    let mut stmt =
        conn.prepare("SELECT id, title, alias, status, episodes_schedule FROM series")?;
    let rows = stmt.query_map([], |row| {
        let id_str: String = row.get(0)?;
        Ok(Series {
            id: Uuid::parse_str(&id_str).unwrap(),
            title: row.get(1)?,
            alias: row.get(2)?,
            status: row.get(3)?,
            episodes_schedule: row.get(4)?,
        })
    })?;

    let mut series = Vec::new();
    for row in rows {
        series.push(row?);
    }
    Ok(series)
}
