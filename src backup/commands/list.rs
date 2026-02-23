use crate::db;
use anyhow::Result;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Status")]
    status: String,
}

pub fn execute() -> Result<()> {
    let conn = db::connect()?;
    let series = db::series::get_all(&conn)?;

    if series.is_empty() {
        println!("No series tracked.");
        return Ok(());
    }

    let rows: Vec<Row> = series
        .into_iter()
        .map(|s| Row {
            id: s.id.to_string()[..8].to_string(),
            title: s.title,
            status: s.status,
        })
        .collect();

    println!("{}", Table::new(rows));
    Ok(())
}
