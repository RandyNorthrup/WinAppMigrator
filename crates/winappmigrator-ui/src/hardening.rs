//! Process hardening for the two elevated, signed-or-signable executables.
//!
//! This is the one place the workspace allows `unsafe_code`: the Windows API call
//! below has no safe wrapper, and it must run before anything else can load a DLL.
#![allow(unsafe_code)]

const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    // Takes flags by value and touches no caller memory, so it is safe to call.
    safe fn SetDefaultDllDirectories(directory_flags: u32) -> i32;
}

/// Resolve DLLs loaded at runtime from System32 only. Windows would otherwise look
/// in the executable's own folder first, so a `d3d12.dll` dropped beside this
/// program in Downloads would run elevated under its publisher's name. Call this
/// first in `main`. Imports listed in the executable itself are covered separately
/// by the `/DEPENDENTLOADFLAG` linker option in each `build.rs`.
pub fn restrict_dll_search() {
    #[cfg(windows)]
    {
        // Failure leaves the default search order. Not fatal, and nothing can be
        // shown yet: no window exists at this point.
        let _succeeded = SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32);
    }
}
