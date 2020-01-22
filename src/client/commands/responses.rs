// Copyright © 2020 Starcat LLC
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
