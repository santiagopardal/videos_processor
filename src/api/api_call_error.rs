use std::fmt::{Debug};

#[derive(Debug)]
pub enum APICallError {
    JSONDecodeError(serde_json::error::Error),
    RequestError(reqwest::Error)
}

impl From<reqwest::Error> for APICallError {
    fn from(value: reqwest::Error) -> Self {
        return APICallError::RequestError(value);
    }
}

impl From<serde_json::error::Error> for APICallError {
    fn from(value: serde_json::error::Error) -> Self {
        return APICallError::JSONDecodeError(value);
    }
}
