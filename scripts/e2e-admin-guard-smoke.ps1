param(
    [string] $WinAppMigratorExe = ".\target\release\winappmigrator.exe",

    [string] $WorkRoot = ".\target\wam-admin-guard-smoke"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = [Security.Principal.WindowsPrincipal]::new($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

if (Test-IsAdministrator) {
    Write-Host "Admin guard smoke is intended for a non-elevated shell; skipping because this shell is elevated."
    exit 0
}

$tool = Resolve-Path -LiteralPath $WinAppMigratorExe
$root = [System.IO.Path]::GetFullPath($WorkRoot)
$packageRoot = Join-Path $root "package"
$targetRoot = Join-Path $root "restore"

if (Test-Path -LiteralPath $root) {
    Remove-Item -LiteralPath $root -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $packageRoot | Out-Null

$manifest = [ordered]@{
    schema_version = "winappmigrator.package.v1"
    source_computer_name = $env:COMPUTERNAME
    source_windows_version = $null
    scan_timestamp_utc = (Get-Date).ToUniversalTime().ToString("o")
    main_executable_path = "C:\Program Files\WinAppMigratorGuard\guard.exe"
    application_metadata = $null
    file_inventory = @()
    registry_keys_exported = @(
        [ordered]@{
            hive = "local_machine"
            key_path = "Software\WinAppMigratorGuard"
            value_name = $null
            value_data = $null
            value_kind = "none"
            value_bytes_hex = $null
            confidence = "high"
            requires_admin = $true
        }
    )
    com_registrations_detected = @()
    file_associations_detected = @()
    services_detected = @()
    drivers_detected = @()
    scheduled_tasks_detected = @()
    shortcuts_detected = @()
    environment_variables_detected = @()
    runtime_prerequisites = @()
    warnings = @()
    items_intentionally_excluded = @("admin guard smoke fixture")
}

$manifestJson = $manifest | ConvertTo-Json -Depth 8
$utf8NoBom = [System.Text.UTF8Encoding]::new($false)
[System.IO.File]::WriteAllText((Join-Path $packageRoot "manifest.json"), $manifestJson, $utf8NoBom)

$previousErrorActionPreference = $ErrorActionPreference
$ErrorActionPreference = "Continue"
try {
    $output = & $tool restore --package $packageRoot --target-root $targetRoot --apply 2>&1
    $exitCode = $LASTEXITCODE
}
finally {
    $ErrorActionPreference = $previousErrorActionPreference
}

if ($exitCode -eq 0) {
    throw "Admin-only restore unexpectedly succeeded from a non-elevated shell."
}

$text = ($output | Out-String)
if ($text -notmatch "admin-only") {
    throw "Admin-only restore failed, but did not produce the expected admin-only message. Output: $text"
}

Write-Host "Admin guard smoke passed. Restore apply was blocked from non-elevated shell."
