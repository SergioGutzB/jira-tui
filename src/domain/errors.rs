use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration Error: {0}")]
    ConfigError(String),

    #[error("Network/API Error: {0}")]
    ApiError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unauthorized operation. Check credentials.")]
    Unauthorized,

    #[error("Input/Output Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Unknown Internal Error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
