use serde::{Deserialize, Serialize};
use static_init::dynamic;

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


pub fn tmdb_api_key() -> &'static str {
    TMDB_API_KEY.as_str()
}