use crate::client::err::ClientError::BadStatus;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Status {
    #[serde(rename = "STATUS_CODE")]
    pub status_code: u32,
    #[serde(rename = "STATUS_TEXT")]
    pub status_text: String,
}

impl Status {
    pub fn ok(&self) -> bool {
        self.status_code == 200
    }

    pub fn assert(&self) -> Result<(), Self> {
        if !self.ok() {
            Err(self.clone())
        } else {
            Ok(())
        }
    }
}

impl From<Status> for Box<dyn std::error::Error> {
    fn from(s: Status) -> Self {
        Box::new(BadStatus(s))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct JSONPayload {
    #[serde(rename = "C")]
    pub code: u32,
    #[serde(rename = "D")]
    pub data: String,
}
