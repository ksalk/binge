use crate::commands::tvmaze;

pub fn execute(name: &str) -> anyhow::Result<()> {
    let results = tvmaze::search(name)?;
    println!("Found {} results for '{}'", results.len(), name);
    Ok(())
}
