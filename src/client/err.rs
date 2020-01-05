use crate::client::commands::responses::Status;

#[derive(Debug)]
pub enum ClientError {
    IOError(std::io::Error),
    DeserializeError(serde_json::error::Error),
    BadStatus(Status),
    Other(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError::IOError(e)
    }
}

impl From<serde_json::error::Error> for ClientError {
    fn from(e: serde_json::error::Error) -> Self {
        ClientError::DeserializeError(e)
    }
}

impl From<Status> for ClientError {
    fn from(s: Status) -> Self {
        ClientError::BadStatus(s)
    }
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO make this nicer
        write!(f, "Client Error")
    }
}
