use std::fmt::{Debug, Formatter};

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

impl Debug for APICallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            APICallError::JSONDecodeError(error) => write!(f, "Error decoding JSON data: {}", error),
            APICallError::RequestError(error) => write!(f, "Error making request to API: {}", error)
        }
    }
}
