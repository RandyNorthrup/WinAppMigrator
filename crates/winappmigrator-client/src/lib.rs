//! The client: runs on the PC an app is moving off (Windows 7, 8, 10 or 11) and lets WinAppMigrator on
//! the new PC read it over the network, through [`winappmigrator_link`]. It captures nothing itself.
//!
//! It takes a shadow copy of the Windows drive through WMI, so what is read is one moment of it and
//! no file is locked, and answers requests to describe, list and read paths in that copy, read-only.
//! It never follows a link there, and only ever reads below the copy's root. While it runs it lets
//! its own two ports through Windows Firewall from the local network only. All of that is undone
//! when it stops, and by the next run if it could not be.

pub mod serve;
pub mod system;

/// The client's version, which the builder is told.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
