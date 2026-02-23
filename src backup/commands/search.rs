use crate::commands::tvmaze::{self, SearchResult};
use anyhow::Result;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "#")]
    id: usize,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Year")]
    year: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Score")]
    score: String,
}

fn to_row(i: usize, result: &SearchResult) -> Row {
    let year = result
        .show
        .premiered
        .as_ref()
        .and_then(|p| p.split('-').next())
        .unwrap_or("????");
    Row {
        id: i,
        title: result.show.name.clone(),
        year: year.to_string(),
        status: result.show.status.clone(),
        score: format!("{:.2}", result.score),
    }
}

pub fn execute(name: &str) -> Result<()> {
    let results = tvmaze::search(name)?;

    let mut filtered: Vec<_> = results.into_iter().filter(|r| r.score > 0.5).collect();
    filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let rows: Vec<Row> = filtered
        .iter()
        .enumerate()
        .map(|(i, result)| to_row(i + 1, result))
        .take(5)
        .collect();

    println!("{}", Table::new(rows));

    Ok(())
}
