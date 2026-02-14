use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("API error ({status}): {message}")]
    Server {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("Connection error: {0}")]
    Connection(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    Deserialization(String),
}
