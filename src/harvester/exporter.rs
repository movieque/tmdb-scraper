use futures::{io::{self, BufReader, ErrorKind}, prelude::*};
use async_compression::futures::bufread::GzipDecoder;
use chrono::{DateTime, Utc, Timelike};
use serde::Deserialize;
use reqwest::Client;
use shared::*;


const HARVESTER_URL: &'static str = "http://files.tmdb.org/p/exports/";


#[derive(Deserialize)]
struct Movie {
    id: u32,
}


pub async fn harvest_export(dataset: Dataset) -> Result<Vec<u32>> {
    let futures = (
        extract(exporter_url(dataset, false)),
        extract(exporter_url(dataset, true)),
    );

    let ids = match dataset {
        Dataset::Movies | Dataset::TvShows | Dataset::People => {
            let (mut ids, adult_ids) = futures::try_join!(futures.0, futures.1)?;
            let iter = adult_ids.iter();
            ids.extend(iter);
            ids
        },
        _ => futures::try_join!(futures.0)?.0,
    };

    Ok(ids)
}


async fn extract(url: String) -> Result<Vec<u32>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let reader = response.bytes_stream().map_err(|e| io::Error::new(ErrorKind::Other, e)).into_async_read();
    let decoder = BufReader::new(GzipDecoder::new(reader));
    let mut lines = decoder.lines();
    let mut ids = Vec::new();
    while let Some(result) = lines.next().await {
        let line = result?;
        let movie: Movie = serde_json::from_str(&line)?;
        ids.push(movie.id);
    }
    Ok(ids)
}

pub fn exporter_url(dataset: Dataset, adult: bool) -> String {
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