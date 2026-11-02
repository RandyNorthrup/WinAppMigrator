//! The installer's logic. It lives in a library so its tests can run without the
//! elevation the `winappmigrator-installer` executable's manifest demands.

pub mod cleanup;
pub mod gui;
pub mod headless;
pub mod uninstall;
mod uninstall_gui;
