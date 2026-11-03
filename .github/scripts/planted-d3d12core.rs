// A harmless stand-in for what a program running as the same account could plant: Direct3D loads a
// file of this name from a folder the environment names, if it says it is newer than Windows' own.
// All this does is say that it ran, in the file a check names. See check-real-windows.ps1.
#![allow(non_snake_case, non_upper_case_globals)]

#[unsafe(no_mangle)]
pub static D3D12SDKVersion: u32 = 999;

#[unsafe(no_mangle)]
pub extern "system" fn DllMain(_module: *mut u8, reason: u32, _reserved: *mut u8) -> i32 {
    const DLL_PROCESS_ATTACH: u32 = 1;
    if reason == DLL_PROCESS_ATTACH {
        if let Some(marker) = std::env::var_os("WAM_PLANTED_MARKER") {
            let who = std::env::current_exe()
                .map(|path| path.display().to_string())
                .unwrap_or_default();
            let _ = std::fs::write(marker, format!("ran in {who}\n"));
        }
    }
    1
}
