<#
.SYNOPSIS
  Full installer end-to-end on a clean, disposable OS using Windows Sandbox.

.DESCRIPTION
  Runs a WinAppMigrator self-extracting installer (built by the app) inside a
  throwaway Windows Sandbox — a clean OS where the target application is NOT
  already installed — with --apply --force, then reads back the installer's own
  SHA-256 verification result. The installer exits non-zero on any hash mismatch,
  so a clean exit plus "Reinstall hashes valid: true" is a strong end-to-end
  assertion of the whole reinstall-on-a-new-machine flow.

  Windows Sandbox gives the LogonCommand an elevated admin session, so the
  installer's privileged work (HKLM registry, services, drivers, prerequisites)
  runs for real, without touching or requiring elevation on the host.

.PREREQUISITES
  Windows 10/11 Pro or Enterprise with the "Windows Sandbox" optional feature
  enabled (one-time, requires elevation + reboot):
      Enable-WindowsOptionalFeature -Online -FeatureName 'Containers-DisposableClientVM' -All

.PARAMETER Installer
  Path to the *_installer.exe produced by the app.

.PARAMETER TargetRoot
  Guest path the app is reinstalled to. Default: C:\Restored.

.PARAMETER RestoreUserData
  Pass --restore-user-data (re-anchor packaged user data to the guest's real
  %AppData%/%ProgramData% instead of staging under TargetRoot).

.PARAMETER TimeoutMinutes
  How long to wait for the guest run to finish. Default: 30.

.EXAMPLE
  pwsh scripts/sandbox-install-e2e.ps1 -Installer C:\out\SolarFire9_userdata_license_installer.exe
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$Installer,
    [string]$TargetRoot = 'C:\Restored',
    [switch]$RestoreUserData,
    [int]$TimeoutMinutes = 30
)

$ErrorActionPreference = 'Stop'

$wsb = Join-Path $env:WINDIR 'System32\WindowsSandbox.exe'
if (-not (Test-Path $wsb)) {
    throw "Windows Sandbox is not enabled. Enable it (elevated, then reboot):`n  Enable-WindowsOptionalFeature -Online -FeatureName 'Containers-DisposableClientVM' -All"
}
if (-not (Test-Path $Installer)) { throw "Installer not found: $Installer" }

$Installer = (Resolve-Path $Installer).Path
$installerDir = Split-Path $Installer -Parent
$installerName = Split-Path $Installer -Leaf

# Host-side work area; the results folder is mapped read-write into the guest.
$work = Join-Path ([System.IO.Path]::GetTempPath()) ("wam-sandbox-" + [System.IO.Path]::GetRandomFileName().Substring(0, 8))
$results = Join-Path $work 'results'
New-Item -ItemType Directory -Force -Path $results | Out-Null

$userDataFlag = if ($RestoreUserData) { ' --restore-user-data' } else { '' }

# Guest script: run the installer with --apply, capture output + exit code, then
# raise a done flag the host polls for.
$guestCmd = @"
@echo off
"C:\installer\$installerName" --target-root "$TargetRoot" --apply --force$userDataFlag > C:\results\output.txt 2>&1
echo %ERRORLEVEL% > C:\results\exitcode.txt
echo done > C:\results\done.flag
"@
$guestCmdPath = Join-Path $results 'run.cmd'
Set-Content -Path $guestCmdPath -Value $guestCmd -Encoding Ascii

$wsbXml = @"
<Configuration>
  <MappedFolders>
    <MappedFolder><HostFolder>$installerDir</HostFolder><SandboxFolder>C:\installer</SandboxFolder><ReadOnly>true</ReadOnly></MappedFolder>
    <MappedFolder><HostFolder>$results</HostFolder><SandboxFolder>C:\results</SandboxFolder><ReadOnly>false</ReadOnly></MappedFolder>
  </MappedFolders>
  <LogonCommand><Command>cmd.exe /c C:\results\run.cmd</Command></LogonCommand>
  <MemoryInMB>8192</MemoryInMB>
</Configuration>
"@
$wsbPath = Join-Path $work 'install-e2e.wsb'
Set-Content -Path $wsbPath -Value $wsbXml -Encoding UTF8

Write-Host "Launching Windows Sandbox to install '$installerName' on a clean OS..."
Write-Host "  results: $results"
& $wsb $wsbPath

$donePath = Join-Path $results 'done.flag'
$deadline = (Get-Date).AddMinutes($TimeoutMinutes)
while (-not (Test-Path $donePath)) {
    if ((Get-Date) -gt $deadline) {
        throw "Timed out after $TimeoutMinutes min waiting for the guest install. Inspect $results."
    }
    Start-Sleep -Seconds 5
}

$output = Get-Content (Join-Path $results 'output.txt') -Raw -ErrorAction SilentlyContinue
$exit = (Get-Content (Join-Path $results 'exitcode.txt') -Raw -ErrorAction SilentlyContinue).Trim()
Write-Host "`n----- guest installer output -----`n$output`n----------------------------------"
Write-Host "guest exit code: $exit"

$hashesOk = $output -match 'Reinstall hashes valid:\s*true'
if ($exit -eq '0' -and $hashesOk) {
    Write-Host 'E2E PASS: clean-OS reinstall + hash verification succeeded.' -ForegroundColor Green
    exit 0
}
else {
    Write-Host 'E2E FAIL: reinstall or hash verification did not pass on the clean OS.' -ForegroundColor Red
    exit 1
}
