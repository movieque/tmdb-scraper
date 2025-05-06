use chrono::{DateTime, Utc, Timelike};
use super::*;

pub fn harvester_url(dataset: Dataset, adult: bool) -> String {
    let base_url = HARVESTER_URL;
    let adult = if adult {"adult_"}else{""};
    let dataset = dataset.as_ref();
    let date = processing_date();
    let url = format!("{}{}{}{}.json.gz", base_url, adult, dataset, date);
    url
}


fn processing_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    let now = if now.hour() < 8 {
        now - chrono::Duration::days(1)
    } else {
        now
    };
    now.format("%m_%d_%Y").to_string()
}