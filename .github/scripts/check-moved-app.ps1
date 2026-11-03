# The second half of a real move between two PCs. This runner has never seen the app, builds nothing,
# and has nothing of this project's to run but this script and the installers it was handed. It
# installs them and looks at the result with Windows' own tools. See
# crates/winappmigrator-installer/tests/elevated_move.rs.
#
# Run only by the job that downloads those installers, on a runner that is thrown away afterward.
param([Parameter(Mandatory)][string]$Folder)

$ErrorActionPreference = 'Stop'
$script:failures = [System.Collections.Generic.List[string]]::new()

function Check([string]$Label, [bool]$Passed, [string]$Detail = '') {
    if ($Passed) {
        Write-Host "  ok: $Label"
    } else {
        Write-Host "  FAILED: $Label`: $Detail"
        $script:failures.Add("$Label`: $Detail")
    }
}

function Section([string]$Title) { Write-Host "`n=== $Title" }

. (Join-Path $PSScriptRoot 'run-program.ps1')

# And says here what it did.
function Run-And-Show([string]$Program, [string[]]$Arguments) {
    Write-Host "`n--- $Program $($Arguments -join ' ')"
    $run = Run-Program $Program $Arguments
    Write-Host "exit $($run.Code)"
    Write-Host $run.Output
    $run
}

function Install-Id([string]$Output) {
    foreach ($line in $Output -split "`r?`n") {
        if ($line -match '^Install id: (.+)$') { return $Matches[1].Trim() }
    }
    throw 'the installer did not say its install id'
}

function Uninstaller-Of([string]$Id) { "C:\ProgramData\WinAppMigrator\installs\$Id\uninstall.exe" }

function Check-Files($Files, [string]$Root, [string]$What) {
    foreach ($file in $Files) {
        # By the full form of the path, which is a file whatever this Windows makes of a name like aux.h.
        $path = '\\?\' + (Join-Path $Root $file.relative)
        if (-not [IO.File]::Exists($path)) {
            Check "$What`: $($file.relative) is there" $false 'it is not'
            continue
        }
        $stream = [IO.File]::OpenRead($path)
        $hash = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($stream)).ToLowerInvariant()
        $stream.Dispose()
        Check ".NET SHA-256, $What`: $($file.relative)" ($hash -eq $file.sha256) "$hash, expected $($file.sha256)"
    }
}

$expected = Get-Content -Raw -LiteralPath (Join-Path $Folder 'expected.json') | ConvertFrom-Json
$versionKey = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
$build = [int]$versionKey.CurrentBuildNumber
$architecture = (Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment').PROCESSOR_ARCHITECTURE
Write-Host "This PC: $($versionKey.ProductName), build $build.$($versionKey.UBR), $architecture"
Write-Host "Built on: build $($expected.source_build)"
Write-Host "Elevated: $(([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole('Administrators'))"

$sourceFolder = "C:\Program Files\$($expected.app.name)"
Check 'this PC has never had the app' (-not (Test-Path -LiteralPath $sourceFolder)) $sourceFolder
Check 'this PC has none of its service' ($null -eq (Get-Service -Name $expected.app.service -ErrorAction SilentlyContinue)) 'it has one'

Section 'What Windows here does with a file named for a device'
$probeFolder = Join-Path $env:RUNNER_TEMP 'device-name-probe'
New-Item -ItemType Directory -Force -Path $probeFolder | Out-Null
$probe = Join-Path $probeFolder 'aux.h'
$opensAsDevice = $false
try {
    [IO.File]::WriteAllText($probe, 'written to a file')
    $listed = @(Get-ChildItem -LiteralPath $probeFolder -Force | ForEach-Object Name)
    Write-Host "writing to aux.h did not fail. The folder now lists: [$($listed -join ', ')]"
    $opensAsDevice = -not ($listed -contains 'aux.h')
} catch {
    Write-Host "writing to aux.h failed: $($_.Exception.Message)"
    $opensAsDevice = $true
}
Write-Host "On build $build, a file named aux.h opens as: $(if ($opensAsDevice) { 'a device' } else { 'a file' })"

Section 'A preview changes nothing'
$installer = Join-Path $Folder $expected.app.installer
$target = 'C:\Program Files\WAM Move Restored'
$run = Run-And-Show $installer @('--target-root', $target)
Check 'the preview succeeded' ($run.Code -eq 0) $run.Output
Check 'it made no install location' (-not (Test-Path -LiteralPath $target)) $target
Check 'it says how much room it needs' ($run.Output -match 'Room: up to') $run.Output

Section 'Install the whole app'
$run = Run-And-Show $installer @('--target-root', $target, '--apply')
Check 'it installs' ($run.Code -eq 0) $run.Output
Check 'it read back what it installed' ($run.Output -match 'Reinstall hashes valid: true') $run.Output
Check-Files $expected.app.files $target 'the installed app'

# What differs between one PC and another is what this is for.
if ($build -lt $expected.source_build) {
    Check 'it says this Windows is older than the one the app came from' ($run.Output -match 'Note: This app was on Windows build') $run.Output
} else {
    Check 'it has nothing to say about this Windows being older' ($run.Output -notmatch 'This app was on Windows build') $run.Output
}
if ($architecture -eq 'ARM64') {
    Check 'it says what would run through an emulator' ($run.Output -match 'emulator') $run.Output
} else {
    Check 'it has nothing to say about an emulator' ($run.Output -notmatch 'emulator') $run.Output
}

Section 'What it set up, by Windows own tools'
$program = Join-Path $target $expected.app.program
$started = Run-And-Show $program @('--dry-run')
Check 'the program it installed starts here' ($started.Code -eq 0) $started.Output
$setting = (Get-ItemProperty -LiteralPath $expected.app.registry_key -Name Setting).Setting
Check 'Get-ItemProperty: the setting came along' ($setting -eq 'hello') "$setting"
$service = Get-CimInstance Win32_Service -Filter "Name='$($expected.app.service)'"
Check 'Win32_Service: the service runs the installed program' ($service.PathName -like "*$target*") "$($service.PathName)"
Check 'Win32_Service: one that starts only when asked was not started' ($service.StartMode -eq 'Manual' -and $service.State -eq 'Stopped') "$($service.StartMode), $($service.State)"
$task = Get-ScheduledTask -TaskPath $expected.app.task_folder -TaskName $expected.app.task
Check 'Get-ScheduledTask: the task runs the installed program' (($task.Actions[0].Execute -replace '"', '') -like "$target*") "$($task.Actions[0].Execute)"
$id = Install-Id $run.Output
$entries = Get-ChildItem 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall' | Where-Object { (Get-ItemProperty $_.PSPath).DisplayName -eq $expected.app.name }
Check 'Settings has an entry for it' (@($entries).Count -eq 1) "$(@($entries).Count)"
$uninstallRun = Run-And-Show (Uninstaller-Of $id) @('--uninstall', $id, '--quiet')
Check 'its uninstaller works' ($uninstallRun.Code -eq 0) $uninstallRun.Output
Check 'the install location is gone' (-not (Test-Path -LiteralPath $target)) $target
Check 'the service is gone' ($null -eq (Get-Service -Name $expected.app.service -ErrorAction SilentlyContinue)) 'it is there'
Check 'the task is gone' ($null -eq (Get-ScheduledTask -TaskPath $expected.app.task_folder -TaskName $expected.app.task -ErrorAction SilentlyContinue)) 'it is there'
Check 'the setting is gone' (-not (Test-Path -LiteralPath $expected.app.registry_key)) 'it is there'

Section 'An installer of two files, copied together to a folder of their own'
$twoFolder = 'C:\WAM Move Two'
New-Item -ItemType Directory -Force -Path $twoFolder | Out-Null
Copy-Item -LiteralPath (Join-Path $Folder $expected.two_files.installer) -Destination $twoFolder
$twoInstaller = Join-Path $twoFolder $expected.two_files.installer
$twoTarget = 'C:\Program Files\WAM Move Two Restored'
$run = Run-And-Show $twoInstaller @('--target-root', $twoTarget, '--apply')
Check 'without its data file, it is refused and says what to do' (($run.Code -ne 0) -and ($run.Output -match 'stay together')) $run.Output
Check 'and made nothing' (-not (Test-Path -LiteralPath $twoTarget)) $twoTarget
Copy-Item -LiteralPath (Join-Path $Folder $expected.two_files.data_file) -Destination $twoFolder
$run = Run-And-Show $twoInstaller @('--target-root', $twoTarget, '--apply')
Check 'with it, it installs' ($run.Code -eq 0) $run.Output
Check-Files $expected.two_files.files $twoTarget 'the two-file install'
$uninstallRun = Run-And-Show (Uninstaller-Of (Install-Id $run.Output)) @('--uninstall', (Install-Id $run.Output), '--quiet')
Check 'it uninstalls' ($uninstallRun.Code -eq 0) $uninstallRun.Output

Section 'An installer holding a file named for a device'
$deviceInstaller = Join-Path $Folder $expected.device_name_app.installer
$deviceTarget = 'C:\Program Files\WAM Move Device Restored'
$installsBefore = @(Get-ChildItem -LiteralPath 'C:\ProgramData\WinAppMigrator\installs' -ErrorAction SilentlyContinue | ForEach-Object Name)
$run = Run-And-Show $deviceInstaller @('--target-root', $deviceTarget, '--apply')
if ($opensAsDevice) {
    Check 'where that name opens as a device, it stops and says so' (($run.Code -ne 0) -and ($run.Output -match 'opens as a device')) $run.Output
    Check 'and made nothing' (-not (Test-Path -LiteralPath $deviceTarget)) $deviceTarget
    $installsAfter = @(Get-ChildItem -LiteralPath 'C:\ProgramData\WinAppMigrator\installs' -ErrorAction SilentlyContinue | ForEach-Object Name)
    Check 'and started no record of an install' (($installsAfter -join ',') -eq ($installsBefore -join ',')) ($installsAfter -join ',')
} else {
    Check 'where that name is a file, it installs' ($run.Code -eq 0) $run.Output
    Check-Files $expected.device_name_app.files $deviceTarget 'the app with a file named for a device'
}

Write-Host "`nSummary: build $build, $architecture, $($script:failures.Count) failures."
if ($script:failures.Count -gt 0) {
    $script:failures | ForEach-Object { Write-Host "  $_" }
    exit 1
}
# Or the job goes by the last program's exit code, and where the last installer here is refused, that
# is what was hoped for.
exit 0
