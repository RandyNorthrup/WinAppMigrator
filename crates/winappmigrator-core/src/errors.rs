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

    #[error("it was cancelled")]
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

/// What a panic said, for saying so instead of losing it. A panic takes a thread with it, and a window
/// waiting on that thread would wait forever, so what runs a long job on one catches it.
pub fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|text| (*text).to_owned())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "no reason was given".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_a_panic_said_can_be_told() {
        assert_eq!(panic_text(&"plain"), "plain");
        assert_eq!(panic_text(&"built".to_owned()), "built");
        assert_eq!(panic_text(&7_u32), "no reason was given");
    }
}
