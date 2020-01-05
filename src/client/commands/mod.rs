use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct Command {}

#[derive(Deserialize, Clone, Debug)]
pub(super) struct NoOp {
    #[serde(rename = "STATUS_CODE")]
    pub status_code: u32,
    #[serde(rename = "STATUS_TEXT")]
    pub status_text: String,
}

#[derive(Deserialize, Clone, Debug)]
pub(super) struct SamplePayload {
    #[serde(rename = "DATA")]
    pub data: String,
}

#[derive(Serialize)]
pub(super) struct NoArgs;
