use crate::commands::tvmaze::{self, SearchResult};
use crate::db::{self, series::Series};
use anyhow::{bail, Result};
use std::io::{self, BufRead, Write};
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

fn fetch_results(name: &str) -> Result<Vec<SearchResult>> {
    let results = tvmaze::search(name)?;
    let mut filtered: Vec<_> = results.into_iter().filter(|r| r.score > 0.5).collect();
    filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    Ok(filtered)
}

fn display_table(results: &[SearchResult]) {
    let rows: Vec<Row> = results
        .iter()
        .enumerate()
        .map(|(i, r)| Row {
            id: i + 1,
            title: r.show.name.clone(),
            year: r
                .show
                .premiered
                .as_ref()
                .and_then(|p| p.split('-').next())
                .unwrap_or("????")
                .to_string(),
            status: r.show.status.clone(),
            score: format!("{:.2}", r.score),
        })
        .take(5)
        .collect();
    println!("{}", Table::new(rows));
}

fn prompt_selection(max: usize) -> Result<Option<usize>> {
    print!("\nEnter # to add (or q to quit): ");
    io::stdout().flush()?;

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    let line = line.trim();

    if line.eq_ignore_ascii_case("q") {
        return Ok(None);
    }

    let id: usize = line.parse()?;
    if id == 0 || id > max {
        bail!("Invalid selection: {}", id);
    }

    Ok(Some(id - 1))
}

fn save_series(result: &SearchResult) -> Result<Series> {
    let series = Series::new(result.show.name.clone(), None, result.show.status.clone());

    let conn = db::connect()?;
    db::series::insert(&conn, &series)?;

    Ok(series)
}

pub fn execute(name: &str) -> Result<()> {
    let results = fetch_results(name)?;
    let display_results: Vec<_> = results.into_iter().take(5).collect();

    display_table(&display_results);

    let Some(index) = prompt_selection(display_results.len())? else {
        println!("Cancelled.");
        return Ok(());
    };

    let series = save_series(&display_results[index])?;
    println!("\nAdded: {} [{}]", series.title, series.status);
    println!("ID: {}", series.id);

    Ok(())
}
