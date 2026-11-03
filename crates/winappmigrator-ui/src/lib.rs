//! Shared egui theme and widget toolkit for the WinAppMigrator GUIs.
//!
//! Both the main WinAppMigrator app and the self-extracting installer render
//! with the same polished, dark/light theme and widget set. Keeping them in one
//! crate means the two front ends stay visually identical without duplication.

pub mod hardening;
pub mod theme;
pub mod widgets;
pub mod window;

/// What a person is told of why a window could not open, whatever the error was.
pub const NEEDS_DIRECT3D_12_TEXT: &str = "This needs Direct3D 12 to draw its window, which an old \
     graphics driver or a virtual machine may not have.";

/// Which graphics adapter a window is drawn with, for the log. On a PC with no graphics card it is
/// Windows' own software one, which is slower and is why a screen may be.
pub fn graphics_adapter_of(creation: &eframe::CreationContext<'_>) -> String {
    creation.wgpu_render_state.as_ref().map_or_else(
        || "none".to_owned(),
        |state| {
            let info = state.adapter.get_info();
            format!("{}, {:?}, {:?}", info.name, info.device_type, info.backend)
        },
    )
}
