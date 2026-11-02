//! Lets a front end follow an installer build and cancel it.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::errors::{Result, WinAppMigratorError};

/// The step an installer build has reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildStep {
    /// Reverse-engineering the application's footprint. Not cancellable part-way,
    /// so a cancel takes effect when this step ends.
    Scanning,
    /// Copying the application's files into the package.
    Copying { done: usize, total: usize },
    /// Downloading runtime prerequisites and bundling drivers.
    Bundling,
    /// Compressing the package.
    Compressing { done: usize, total: usize },
    /// Combining the installer stub and the package into the final executable.
    WritingInstaller,
}

type Observer = Arc<dyn Fn(BuildStep) + Send + Sync>;

/// Reports build progress and carries the request to cancel. Clones share the same
/// cancel flag, so the UI thread can cancel a build running on a worker thread.
#[derive(Clone, Default)]
pub struct BuildMonitor {
    observer: Option<Observer>,
    cancelled: Arc<AtomicBool>,
}

impl BuildMonitor {
    pub fn new(observer: impl Fn(BuildStep) + Send + Sync + 'static) -> Self {
        Self {
            observer: Some(Arc::new(observer)),
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub(crate) fn report(&self, step: BuildStep) {
        if let Some(observer) = self.observer.as_ref() {
            observer(step);
        }
    }

    /// Stop the build with [`WinAppMigratorError::Cancelled`] if it was cancelled.
    pub(crate) fn checkpoint(&self) -> Result<()> {
        if self.is_cancelled() {
            return Err(WinAppMigratorError::Cancelled);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn clones_share_the_cancel_flag() {
        let monitor = BuildMonitor::default();
        let handle = monitor.clone();

        assert!(monitor.checkpoint().is_ok());
        handle.cancel();

        assert!(monitor.is_cancelled());
        assert!(matches!(
            monitor.checkpoint(),
            Err(WinAppMigratorError::Cancelled)
        ));
    }

    #[test]
    fn observer_receives_each_step() -> Result<()> {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let monitor = BuildMonitor::new(move |step| {
            if let Ok(mut steps) = sink.lock() {
                steps.push(step);
            }
        });

        monitor.report(BuildStep::Scanning);
        monitor.report(BuildStep::Copying { done: 1, total: 2 });

        let steps = seen
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert_eq!(
            *steps,
            vec![
                BuildStep::Scanning,
                BuildStep::Copying { done: 1, total: 2 }
            ]
        );
        Ok(())
    }
}
