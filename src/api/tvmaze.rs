use serde::Deserialize;
use reqwest::blocking::Client;

#[derive(Deserialize)]
pub struct SearchResult {
    pub score: f64,
    pub show: Show,
}

#[derive(Deserialize)]
pub struct Show {
    pub id: u32,
    pub name: String,
    pub premiered: Option<String>,
    pub status: String,
}

pub fn search(name: &str) -> Result<Vec<SearchResult>, String> {
    let base_url = "https://api.tvmaze.com";
    let search_endpoint = format!("{}/search/shows?q={name}", base_url);

    let response = Client::new()
        .get(search_endpoint)
        .send().expect("Error during HTTP request")
        .text().expect("Error during getting HTTP response");

    let found_series: Vec<SearchResult> = serde_json::from_str(&response).expect("Error during JSON deserialization");

    Ok(found_series)
}