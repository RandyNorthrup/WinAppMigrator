// Always a windowed GUI app — never allocate a console window, in any build.
#![windows_subsystem = "windows"]

fn main() -> eframe::Result {
    // First, before anything else can load a DLL.
    winappmigrator_ui::hardening::restrict_dll_search();
    winappmigrator_gui::run()
}
