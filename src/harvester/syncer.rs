use std::collections::HashSet;
use chrono::{Utc, Duration};
use serde::Deserialize;
use reqwest::Client;
use shared::*;


const MOVIE_CHANGES_URL: &'static str = "https://api.themoviedb.org/3/movie/changes";
const PEOPLE_CHANGES_URL: &'static str = "https://api.themoviedb.org/3/person/changes";
const TVSHOW_CHANGES_URL: &'static str = "https://api.themoviedb.org/3/tv/changes";


#[derive(Deserialize)]
struct Response {
    results: Vec<Object>,
    total_pages: u16,
}

#[derive(Deserialize)]
struct Object {
    id: u32,
}


pub async fn harvest_sync(dataset: Dataset, days_interval: u8) -> Result<Vec<u32>> {
    let url = url(&dataset)?;
    let api_key = api_key();
    let client = Client::new();
    let (ids, pages) = request(url, 1, api_key, &client, days_interval).await?;
    let mut set = ids.into_iter().collect::<HashSet<u32>>();
    let mut futures = Vec::new();
    for page in 2..=pages {
        let mut batch = Vec::new();
        for _ in 0..25 {
            batch.push(request(url, page, api_key, &client, days_interval))
        }
        futures.push(batch);
    }

    for iter in futures {
        let result = futures::future::try_join_all(iter).await?;
        for (iter, _) in result {
            set.extend(iter);
        }
    }

    Ok(set.into_iter().collect::<Vec<u32>>())
}


async fn request(url: &str, page: u16, api_key: &str, client: &Client, days_interval: u8) -> Result<(Vec<u32>, u16)> {
    let page = page.to_string();
    let start_date = (Utc::now() - Duration::days(days_interval as i64)).format("%Y-%m-%d").to_string();
    let end_date = (Utc::now() + Duration::days(1)).format("%Y-%m-%d").to_string();
    let response = client.get(url).query(&[("api_key", api_key), ("page", &page), ("start_date", &start_date), ("end_date", &end_date)]).send().await?.json::<Response>().await?;
    let mut set = Vec::new();
    for object in &response.results {
        set.push(object.id);
    }
    let total_pages = response.total_pages;
    Ok((set, total_pages))
}


fn url(dataset: &Dataset) -> Result<&'static str> {
    match dataset {
        Dataset::Movies => Ok(MOVIE_CHANGES_URL),
        Dataset::People => Ok(PEOPLE_CHANGES_URL),
        Dataset::TvShows => Ok(TVSHOW_CHANGES_URL),
        _ => Err("Invalid dataset for sync".into()),
    }
}

