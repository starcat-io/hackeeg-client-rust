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

use crate::client::commands::responses::Status;
use base64::DecodeError;

#[derive(Debug)]
pub enum ClientError {
    IOError(std::io::Error),
    DeserializeError(Box<dyn std::error::Error>),
    BadStatus(Status),
    InvalidBase64(base64::DecodeError),
    Other(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        ClientError::IOError(e)
    }
}

impl From<serde_json::error::Error> for ClientError {
    fn from(e: serde_json::error::Error) -> Self {
        ClientError::DeserializeError(Box::new(e))
    }
}

impl From<rmp_serde::decode::Error> for ClientError {
    fn from(e: rmp_serde::decode::Error) -> Self {
        ClientError::DeserializeError(Box::new(e))
    }
}

impl From<Status> for ClientError {
    fn from(s: Status) -> Self {
        ClientError::BadStatus(s)
    }
}

impl From<base64::DecodeError> for ClientError {
    fn from(e: base64::DecodeError) -> Self {
        ClientError::InvalidBase64(e)
    }
}

impl std::error::Error for ClientError {}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        // TODO make this nicer
        write!(f, "Client Error")
    }
}
