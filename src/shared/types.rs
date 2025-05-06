use aws_sdk_sqs::types::SendMessageBatchRequestEntry;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
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


impl Display for Dataset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dataset::Movies => write!(f, "movies"),
            Dataset::TvShows => write!(f, "tv shows"),
            Dataset::Seasons => write!(f, "seasons"),
            Dataset::Episodes => write!(f, "episodes"),
            Dataset::People => write!(f, "people"),
            Dataset::Networks => write!(f, "networks"),
            Dataset::Companies => write!(f, "companies")
        }
    }
}

/// This is only to be used when constructing the TMDB daily ID export url.
impl AsRef<str> for Dataset {
    fn as_ref(&self) -> &str {
        match self {
            Dataset::Movies => "movie_ids_",
            Dataset::TvShows => "tv_series_ids_",
            Dataset::Seasons => "seasons_ids_",
            Dataset::Episodes => "episodes_ids_",
            Dataset::People => "person_ids_",
            Dataset::Networks => "tv_network_ids_",
            Dataset::Companies => "production_company_ids_"
        }
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub dataset: Dataset,
    pub ids: Vec<u32>,
}

impl TryFrom<Message> for SendMessageBatchRequestEntry {
    type Error = Box<dyn Error + Send + Sync + 'static>;

    fn try_from(message: Message) -> Result<Self> {
        let id = message.dataset.to_string() + "-" + &message.ids.first().ok_or("cannot create a message from empty ids")?.to_string() + & match message.ids.last(){
            Some(last) => String::from("-") + &last.to_string(),
            None => String::new()
        };
        let message_body = serde_json::to_string(&message)?;
        let message = SendMessageBatchRequestEntry::builder().id(id).message_body(message_body).build()?;
        Ok(message)
    }
}