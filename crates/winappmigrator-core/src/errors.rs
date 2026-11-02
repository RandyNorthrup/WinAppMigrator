use std::io;
use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, WinAppMigratorError>;

#[derive(Debug, Error)]
pub enum WinAppMigratorError {
    #[error("I/O failed for {path}: {source}")]
    Io { path: PathBuf, source: io::Error },

    #[error("registry read failed: {0}")]
    Registry(String),

    #[error("PE parse failed for {path}: {detail}")]
    Pe { path: PathBuf, detail: String },

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("report serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("archive operation failed: {0}")]
    Archive(String),

    #[error("network operation failed: {0}")]
    Network(String),

    #[error("time formatting failed: {0}")]
    TimeFormat(#[from] time::error::Format),

    #[error("the build was cancelled")]
    Cancelled,
}

impl WinAppMigratorError {
    pub fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    pub fn pe(path: impl Into<PathBuf>, source: impl Into<String>) -> Self {
        Self::Pe {
            path: path.into(),
            detail: source.into(),
        }
    }
}
