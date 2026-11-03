# What an account has for an app, with the User data option on. elevated_real_apps.rs has built 7-Zip
# and PuTTY with that on, on this runner, from what install-real-apps.ps1 left in this account's keys.
# This takes the apps and those keys away, shows that they are gone, installs what was built, shows that
# each value came back to the account installing it exactly as it was, and runs the uninstall string
# each one recorded, which has to take them away again.
#
# Run only by the job that built those installers, on a runner that is thrown away afterward.
param([Parameter(Mandatory)][string]$Folder)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'real-app-checks.ps1')
Require-ThrowawayRunner

$script:failures = [Collections.Generic.List[string]]::new()

function Check([string]$Label, [bool]$Passed, [string]$Detail = '') {
    if ($Passed) {
        Write-Host "  ok: $Label"
    } else {
        Write-Host "  FAILED: $Label`: $Detail"
        $script:failures.Add("$Label`: $Detail")
    }
}

$list = Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'real-apps.json') | ConvertFrom-Json
$expected = Get-Content -Raw -LiteralPath (Join-Path $Folder 'expected.json') | ConvertFrom-Json
$settings = @($list.account_settings | Where-Object { $_.goes_with -eq 'user_data' })
$files = @($list.account_files | Where-Object { $_.goes_with -eq 'user_data' })

Write-Host "`n=== Take the apps away, and what this account has of them"
foreach ($app in @($list.apps | Where-Object { $_.id -in $expected.apps.id })) {
    Remove-Preinstalled $app
    Stop-ProcessesFrom $app.installed_at
}
foreach ($key in 'Software\7-Zip', 'Software\SimonTatham') {
    [Microsoft.Win32.Registry]::CurrentUser.DeleteSubKeyTree($key, $false)
    Write-Host "  HKCU\$key is gone"
}
foreach ($setting in $settings) {
    Check "with it gone, it fails as it should: $($setting.key)\$($setting.name)" ([bool](AccountSetting-Problem $setting)) 'it is still there'
}
foreach ($file in $files) {
    Remove-Item -LiteralPath (Split-Path -Parent (AccountFile-Path $file)) -Recurse -Force
    Check "with it gone, it fails as it should: $($file.relative)" (-not (Test-Path -LiteralPath (AccountFile-Path $file))) 'it is still there'
}

$installs = @{}
foreach ($app in @($expected.apps)) {
    Write-Host "`n=== Install $($app.name) with this account's data"
    $run = Run-Program (Join-Path $Folder $app.installer) @('--apply', '--restore-user-data')
    Write-Host "exit $($run.Code)"
    Write-Host $run.Output
    Check "$($app.id): the installer exits with 0" ($run.Code -eq 0) $run.Output
    Check "$($app.id): no errors" ($run.Output -match '(?m)^Errors: 0\s*$') ''
    if ($run.Output -match '(?m)^Install id: (.+?)\s*$') { $installs[$app.id] = $Matches[1] }
}

Write-Host "`n=== Each value came to the account installing, as it was"
foreach ($setting in $settings) {
    $problem = AccountSetting-Problem $setting
    Check "$($setting.key)\$($setting.name) ($($setting.kind))" (-not $problem) $problem
}

foreach ($file in $files) {
    $path = AccountFile-Path $file
    Check "$($file.relative) is in this account's folders, as it was" ((Test-Path -LiteralPath $path) -and (Get-Content -Raw -LiteralPath $path) -ceq $file.content) $path
}

foreach ($app in @($expected.apps)) {
    Write-Host "`n=== Uninstall $($app.name) by what it recorded"
    $entries = @(Uninstall-Entries $app.name)
    if ($entries.Count -ne 1) {
        Check "$($app.id): one entry to uninstall by" $false "$($entries.Count)"
        continue
    }
    $command = Split-CommandLine $entries[0].QuietUninstall
    $run = Run-Program $command.Program $command.Arguments
    Write-Host "exit $($run.Code)"
    Write-Host $run.Output
    Check "$($app.id): its uninstall string works" ($run.Code -eq 0) $run.Output
    $record = "C:\ProgramData\WinAppMigrator\installs\$($installs[$app.id])"
    Check "$($app.id): its record goes" (Wait-Until { -not (Test-Path -LiteralPath $record) } 60) $record
}

Write-Host "`n=== And they went with it"
foreach ($setting in $settings) {
    Check "gone again: $($setting.key)\$($setting.name)" ([bool](AccountSetting-Problem $setting)) 'it is still there'
}

Write-Host "`nSummary: $($script:failures.Count) failures."
if ($script:failures.Count -gt 0) {
    $script:failures | ForEach-Object { Write-Host "  $_" }
    exit 1
}
exit 0
