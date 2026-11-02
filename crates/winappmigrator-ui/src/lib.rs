//! Shared egui theme and widget toolkit for the WinAppMigrator GUIs.
//!
//! Both the main WinAppMigrator app and the self-extracting installer render
//! with the same polished, dark/light theme and widget set. Keeping them in one
//! crate means the two front ends stay visually identical without duplication.

pub mod hardening;
pub mod theme;
pub mod widgets;
