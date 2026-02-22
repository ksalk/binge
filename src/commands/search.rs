use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchResult {
    score: f64,
    show: Show,
}

#[derive(Deserialize)]
struct Show {
    name: String,
    premiered: Option<String>,
    status: String,
}

pub fn execute(name: &str) -> Result<()> {
    let url = format!("https://api.tvmaze.com/search/shows?q={name}");
    let response = reqwest::blocking::get(&url)?.text()?;
    let results: Vec<SearchResult> = serde_json::from_str(&response)?;

    let mut filtered: Vec<_> = results.into_iter().filter(|r| r.score > 0.5).collect();
    filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    for result in filtered.iter().take(5) {
        let year = result
            .show
            .premiered
            .as_ref()
            .and_then(|p| p.split('-').next())
            .unwrap_or("????");
        println!(
            "{} ({}) [{}] - {:.2}",
            result.show.name, year, result.show.status, result.score
        );
    }

    Ok(())
}
