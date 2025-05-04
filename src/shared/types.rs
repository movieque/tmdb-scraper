use serde::{Deserialize, Serialize};
use static_init::dynamic;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Action {
    Export(Dataset),
    Sync(Dataset)
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Dataset {
    Movies,
    TvShows,
    Seasons,
    Episodes,
    People,
    Networks,
    Companies
}

#[dynamic]
static TMDB_API_KEY: String = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY not set");

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;


pub fn tmdb_api_key() -> &'static str {
    TMDB_API_KEY.as_str()
}