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
}

#[derive(Deserialize, Clone, Debug)]
pub struct Sample {
    #[serde(rename = "DATA")]
    pub data: String,
}
