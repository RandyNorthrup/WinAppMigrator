//! The encrypted link between WinAppMigrator on a new PC (the builder) and its client on an old one.
//!
//! The client runs on the PC an app is moving off, which can be Windows 7, 8, 10 or 11. It does no
//! capturing of its own: it takes a snapshot of that PC's Windows drive and answers the builder's
//! requests to list a folder, describe a file, and read part of one, read-only. The builder captures
//! the app through those answers as it would from a drive plugged into it.
//!
//! - [`discovery`]: how the builder finds clients on its network, over UDP.
//! - [`channel`]: pairing by the short code the client shows, with SPAKE2, so the code never crosses
//!   the network and cannot be guessed from what does, then ChaCha20-Poly1305 for everything after.
//! - [`protocol`]: what is asked and answered.
//! - [`paths`]: which paths may be asked about at all.
//!
//! This crate builds for Windows 7 as well as 10 and 11, so it uses nothing newer than both have.

pub mod channel;
pub mod discovery;
pub mod paths;
pub mod protocol;

use std::io;

/// Changes whenever either side would not understand the other.
pub const PROTOCOL_VERSION: u32 = 1;
/// Where clients listen for a builder asking who is there.
pub const DISCOVERY_PORT: u16 = 47731;
/// Where a client takes a builder's connection.
pub const LINK_PORT: u16 = 47732;
/// What every message of this protocol starts with, so nothing else is taken for one.
pub const MAGIC: &str = "winappmigrator-link";
/// How many digits a pairing code has. With SPAKE2, someone without it gets one guess per attempt,
/// and a client gives a code a few attempts before it makes another.
pub const CODE_DIGITS: usize = 6;

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("the code was not right, or the other PC stopped")]
    WrongCode,
    #[error("the other side does not speak this version of the link: {0}")]
    Protocol(String),
    #[error(
        "a message could not be read: it was changed on the way, or is not from the other side"
    )]
    Tampered,
    #[error("the other side closed the link")]
    Closed,
}

pub type Result<T> = std::result::Result<T, LinkError>;
