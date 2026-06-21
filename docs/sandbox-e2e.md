# Clean-OS install end-to-end (Windows Sandbox)

The unit suite and `crates/winappmigrator-cli/tests/installer_e2e.rs` build an
installer and reinstall it into a *directory* on the build machine. To exercise
the full "reinstall on a different machine" flow — privileged registry/service/
driver/prerequisite work included — run the produced installer on a **clean,
disposable OS** with [`scripts/sandbox-install-e2e.ps1`](../scripts/sandbox-install-e2e.ps1).

It uses **Windows Sandbox**: a throwaway Windows VM where the target application
is not already installed and the logon session is elevated, so the installer's
`--apply` does its real privileged work without touching or needing elevation on
the host.

## One-time setup

Windows 10/11 Pro or Enterprise, elevated, then reboot:

```powershell
Enable-WindowsOptionalFeature -Online -FeatureName 'Containers-DisposableClientVM' -All
```

## Run

```powershell
# 1. Build the installer in the app (or any create_package caller). It produces
#    e.g. C:\out\SolarFire9_userdata_license_installer.exe
# 2. Drive a clean-OS reinstall + hash verification:
pwsh scripts/sandbox-install-e2e.ps1 -Installer C:\out\SolarFire9_userdata_license_installer.exe
```

The script maps the installer into a fresh Sandbox, runs it `--apply --force`,
captures the installer's own SHA-256 verification (`Reinstall hashes valid: true`)
and exit code through a shared folder, and reports **PASS/FAIL**. Add
`-RestoreUserData` to re-anchor packaged user data to the guest's real
`%AppData%`/`%ProgramData%` instead of staging it under the target root.

## Notes

- Machine-bound licenses (hardware dongles, machine-locked activation, DPAPI
  blobs) are restored best-effort but generally require reactivation on the new
  OS — the clean Sandbox is the honest place to confirm that.
- The Sandbox window does not auto-close; results are captured via the shared
  folder regardless, so you can close it once the script reports a verdict.
