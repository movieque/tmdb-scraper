use aws_sdk_sqs::types::BatchResultErrorEntry;
use std::fmt::{Display, Formatter};
use std::error::Error as StdError;

#[derive(Debug, Clone)]
pub enum Error {
    BatchResultErrorEntry(Vec<BatchResultErrorEntry>),
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BatchResultErrorEntry(err) => write!(f, "BatchResultErrorEntry: {:?}", err),
        }
    }
}

impl StdError for Error {}


impl From<Vec<BatchResultErrorEntry>> for Error {
    fn from(err: Vec<BatchResultErrorEntry>) -> Self {
        Error::BatchResultErrorEntry(err)
    }
}