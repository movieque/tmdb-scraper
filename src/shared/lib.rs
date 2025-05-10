mod types;

pub use types::*;

use static_init::dynamic;

#[dynamic]
static API_KEY: String = std::env::var("API_KEY").expect("API_KEY not set");


pub fn api_key() -> &'static str {
    API_KEY.as_str()
}