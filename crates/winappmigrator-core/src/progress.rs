//! Lets a front end follow a long job and cancel it.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::errors::{Result, WinAppMigratorError};
use crate::space::RoomCheck;

/// What a scan is looking at. A scan looks at each of these in turn, and stops between them, and
/// part-way through the ones that can take long, when it is cancelled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStep {
    /// How the app was installed, from its install record.
    InstallRecord,
    /// Its services, drivers, scheduled tasks, shortcuts, and environment variables.
    SystemItems,
    /// What else might be the app's, by name.
    NearMatches,
    /// Its files and folders.
    Files,
    /// What ties it into Windows, in the registry.
    Registry,
    /// What it needs to run, and what it holds that must not be copied.
    Finishing,
}

/// The step an installer build has reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildStep {
    /// Reverse-engineering the application's footprint, for a build that has not been given a scan.
    Scanning,
    /// Applying what a person chose to the scan a build was given.
    Preparing,
    /// How much room this build will need and where, now that it is known. It carries on after
    /// this only if it fits.
    RoomChecked(RoomCheck),
    /// Downloading runtime prerequisites and bundling drivers.
    Bundling,
    /// Reading the application's files into the package, hashing and compressing them as they go.
    Packing {
        done: usize,
        total: usize,
        bytes_done: u64,
        bytes_total: u64,
    },
    /// Finishing the executable, and the data file if there is one.
    WritingInstaller,
}

/// What an install or an uninstall is doing, in the order an install does it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallPhase {
    Unpacking,
    /// Hash-checking the package. Reads all of it, and has no count to give.
    CheckingPackage,
    /// Undoing an earlier install that never finished.
    RecoveringEarlierInstall,
    Prerequisites,
    Files,
    Services,
    Drivers,
    Registry,
    License,
    ScheduledTasks,
    Shortcuts,
    Environment,
    UninstallEntry,
    Verifying,
    /// Putting back what a failed or cancelled install changed.
    RollingBack,
    /// Undoing a finished install.
    Uninstalling,
}

/// How far an install or an uninstall has got. `total` is zero when there is no count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstallStep {
    pub phase: InstallPhase,
    pub done: usize,
    pub total: usize,
}

impl InstallStep {
    pub fn new(phase: InstallPhase, done: usize, total: usize) -> Self {
        Self { phase, done, total }
    }

    /// How much of this phase is done, if it can say.
    pub fn fraction(&self) -> Option<f32> {
        (self.total > 0).then(|| self.done.min(self.total) as f32 / self.total as f32)
    }
}

type Observer<S> = Arc<dyn Fn(S) + Send + Sync>;

/// Reports progress and carries the request to cancel. Clones share the same cancel flag,
/// so the UI thread can cancel a job running on a worker thread.
pub struct Monitor<S> {
    observer: Option<Observer<S>>,
    cancelled: Arc<AtomicBool>,
}

/// Follows the building of an installer.
pub type BuildMonitor = Monitor<BuildStep>;
/// Follows a scan.
pub type ScanMonitor = Monitor<ScanStep>;
/// Follows an install or an uninstall.
pub type InstallMonitor = Monitor<InstallStep>;

impl<S> Clone for Monitor<S> {
    fn clone(&self) -> Self {
        Self {
            observer: self.observer.clone(),
            cancelled: Arc::clone(&self.cancelled),
        }
    }
}

impl<S> Default for Monitor<S> {
    fn default() -> Self {
        Self {
            observer: None,
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl<S> Monitor<S> {
    pub fn new(observer: impl Fn(S) + Send + Sync + 'static) -> Self {
        Self {
            observer: Some(Arc::new(observer)),
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// One that says nothing to anyone, and is cancelled when this is.
    pub(crate) fn sharing_cancel<T>(&self) -> Monitor<T> {
        Monitor {
            observer: None,
            cancelled: Arc::clone(&self.cancelled),
        }
    }

    /// What says it was cancelled, for what has to outlive this.
    pub(crate) fn cancel_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancelled)
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub(crate) fn report(&self, step: S) {
        if let Some(observer) = self.observer.as_ref() {
            observer(step);
        }
    }

    /// Stop the job with [`WinAppMigratorError::Cancelled`] if it was cancelled.
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
    fn a_monitor_sharing_a_cancel_is_cancelled_with_it() {
        let monitor = BuildMonitor::default();
        let scan: ScanMonitor = monitor.sharing_cancel();
        assert!(!scan.is_cancelled());
        monitor.cancel();
        assert!(scan.is_cancelled());
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
        monitor.report(BuildStep::Bundling);

        let steps = seen
            .lock()
            .map_err(|error| WinAppMigratorError::InvalidInput(error.to_string()))?;
        assert_eq!(*steps, vec![BuildStep::Scanning, BuildStep::Bundling]);
        Ok(())
    }

    #[test]
    fn an_install_step_knows_how_far_along_it_is() {
        let step = InstallStep::new(InstallPhase::Files, 1, 4);
        assert_eq!(step.fraction(), Some(0.25));
        assert_eq!(
            InstallStep::new(InstallPhase::CheckingPackage, 0, 0).fraction(),
            None
        );
    }
}
