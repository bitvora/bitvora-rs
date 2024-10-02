use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum APIError {
    Http(StatusCode),
    Reqwest(reqwest::Error),
    Serialization(serde_json::Error),
    BadRequest(StatusCode, String), // New error variant to capture status and response body
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            APIError::Http(code) => write!(f, "HTTP error with status code: {}", code),
            APIError::Reqwest(err) => write!(f, "Request error: {}", err),
            APIError::Serialization(err) => write!(f, "Serialization error: {}", err),
            APIError::BadRequest(status, body) => {
                write!(f, "Bad request ({}): {}", status, body)
            }
        }
    }
}

impl std::error::Error for APIError {}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> Self {
        APIError::Reqwest(err)
    }
}

impl From<serde_json::Error> for APIError {
    fn from(err: serde_json::Error) -> Self {
        APIError::Serialization(err)
    }
}
