use thiserror::Error;



/// ## This is the error class 
/// (Spelling bee error... Typo...)
/// sorry if it was hard to find
#[derive(Error, Debug)]
pub enum Typo {
    #[error("Error trying to send or receive data: {0}")]
    TcpIoError(#[from] std::io::Error),
    #[error("Error while parsing JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Could not find the daily game specified")]
    GetDailyGameNotFound(),
    #[error("CouchDB error: {0}")]
    CouchDbError(#[from] couch_rs::error::CouchError),
}
