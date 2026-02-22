use crate::db;
use anyhow::Result;

pub fn execute() -> Result<()> {
    let conn = db::connect()?;
    db::series::clear(&conn)?;
    println!("Database cleared.");
    Ok(())
}
