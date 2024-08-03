use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Formating error: {0}")]
    Format(#[from] std::fmt::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PDF error: {0}")]
    Pdf(#[from] lopdf::Error),
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for this crate
pub type Result<T> = std::result::Result<T, OutputError>;

impl From<&str> for OutputError {
    fn from(e: &str) -> Self {
        OutputError::Other(e.into())
    }
}
