use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchResult {
    pub score: f64,
    pub show: Show,
}

#[derive(Deserialize)]
pub struct Show {
    pub name: String,
    pub premiered: Option<String>,
    pub status: String,
}

pub fn search(name: &str) -> Result<Vec<SearchResult>> {
    let url = format!("https://api.tvmaze.com/search/shows?q={name}");
    let response = Client::new().get(&url).send()?.text()?;
    let results: Vec<SearchResult> = serde_json::from_str(&response)?;
    Ok(results)
}
