// Always a windowed GUI app — never allocate a console window, in any build.
#![windows_subsystem = "windows"]

fn main() -> eframe::Result {
    // First, before anything else can load a DLL.
    winappmigrator_ui::hardening::restrict_dll_search();
    let result = winappmigrator_gui::run();
    if let Err(error) = result.as_ref() {
        // A windowed program has nowhere to print this, and its window is what failed.
        tracing::error!(%error, "the window could not open");
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("WinAppMigrator")
            .set_description(format!("The window could not open: {error}"))
            .show();
    }
    result
}
