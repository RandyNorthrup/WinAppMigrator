use std::fmt;
use std::io;
use std::path::PathBuf;

use thiserror::Error;
use winappmigrator_i18n::t;

pub type Result<T> = std::result::Result<T, WinAppMigratorError>;

/// What went wrong. Its words are in the catalog, so it reads in the language in use, which is why [`fmt::Display`] is
/// written out below and not made from attributes.
#[derive(Debug, Error)]
pub enum WinAppMigratorError {
    /// What an install would do is not what the person agreed to, so nothing was done.
    ListChanged(Box<crate::restore_review::SystemChanges>),

    Io {
        path: PathBuf,
        source: io::Error,
    },

    Registry(String),

    Pe {
        path: PathBuf,
        detail: String,
    },

    InvalidInput(String),

    /// What was picked has nothing of its own to put in an installer, such as a web app a browser
    /// installed, or a driver with no program. Not a fault: the words say what to do instead.
    NothingToPackage(String),

    Serialization(#[from] serde_json::Error),

    Archive(String),

    Network(String),

    TimeFormat(#[from] time::error::Format),

    Cancelled,
}

impl fmt::Display for WinAppMigratorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::ListChanged(_) => t!("core-error-list-changed"),
            Self::Io { path, source } => t!(
                "core-error-io",
                path = path.display().to_string(),
                error = source.to_string()
            ),
            Self::Registry(detail) => t!("core-error-registry", detail = detail),
            Self::Pe { path, detail } => t!(
                "core-error-pe",
                path = path.display().to_string(),
                detail = detail
            ),
            Self::InvalidInput(detail) => t!("core-error-invalid-input", detail = detail),
            Self::NothingToPackage(explanation) => explanation.clone(),
            Self::Serialization(error) => {
                t!("core-error-serialization", detail = error.to_string())
            }
            Self::Archive(detail) => t!("core-error-archive", detail = detail),
            Self::Network(detail) => t!("core-error-network", detail = detail),
            Self::TimeFormat(error) => t!("core-error-time-format", detail = error.to_string()),
            Self::Cancelled => t!("core-error-cancelled"),
        };
        formatter.write_str(&text)
    }
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
        .unwrap_or_else(|| t!("core-error-panic-no-reason"))
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

    #[test]
    fn an_error_reads_in_english_as_it_always_has() {
        assert_eq!(
            WinAppMigratorError::Cancelled.to_string(),
            "it was cancelled"
        );
        assert_eq!(
            WinAppMigratorError::io(r"C:\App\a.dll", io::Error::other("busy")).to_string(),
            r"I/O failed for C:\App\a.dll: busy"
        );
        assert_eq!(
            WinAppMigratorError::InvalidInput("no such app".to_owned()).to_string(),
            "invalid input: no such app"
        );
        assert_eq!(
            WinAppMigratorError::pe(r"C:\App\a.exe", "too short").to_string(),
            r"PE parse failed for C:\App\a.exe: too short"
        );
    }
}
