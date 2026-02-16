use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("API error ({status}): {message}")]
    Server { status: u16, message: String },

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Failed to parse response: {0}")]
    Deserialization(String),
}
