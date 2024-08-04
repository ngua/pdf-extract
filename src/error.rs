use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Formating error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PDF error: {0}")]
    Pdf(#[from] lopdf::Error),
    #[error("UTF-16 error: {0}")]
    FromUtf16(#[from] std::string::FromUtf16Error),
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for this crate
pub type Result<T> = std::result::Result<T, OutputError>;

impl From<String> for OutputError {
    fn from(e: String) -> Self {
        OutputError::Other(e)
    }
}

impl From<&str> for OutputError {
    fn from(e: &str) -> Self {
        OutputError::Other(e.into())
    }
}
