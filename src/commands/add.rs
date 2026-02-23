use crate::api;

pub fn execute(name: &str) {
    let found_series = api::search(&name).expect("Error while fetching TV series");

    match found_series.first() {
        None => println!("Did not find any series for name {}", name),
        Some(best_matched) => println!("Adding series: {}", best_matched.show.name),
    }
}