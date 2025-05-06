use futures::{io::{self, BufReader, ErrorKind}, prelude::*};
use async_compression::futures::bufread::GzipDecoder;
use serde::Deserialize;
use reqwest::Client;
use shared::*;


#[derive(Deserialize)]
struct Movie {
    id: u32,
}


pub async fn harvest(dataset: Dataset) -> Result<Vec<Message>> {
    let futures = (
        extract(harvester_url(dataset, false)),
        extract(harvester_url(dataset, true)),
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

    let mut sorted = Vec::<Option<u32>>::new();
    for id in ids {
        make_sure_index_exists(&mut sorted, id as usize);
        sorted[id as usize] = Some(id);
    }
    let mut sorted = sorted.into_iter().peekable();
    let mut messages = Vec::<Message>::new();

    while let Some(_) = sorted.peek() {
        let mut ids = Vec::<u32>::new();
        for _ in 0..50 {
            if let Some(Some(id)) = sorted.next() {
                ids.push(id);
            }
        }
        if !ids.is_empty() {
            let message = Message{dataset, ids};
            messages.push(message);
        }
    }

    Ok(messages)
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


fn make_sure_index_exists<T: Default + Clone>(vec: &mut Vec<T>, index: usize) {
    if index >= vec.len() {
        vec.resize(index + 1, T::default());
    }
}