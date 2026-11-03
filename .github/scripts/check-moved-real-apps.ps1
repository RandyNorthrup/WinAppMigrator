# The second half of moving real apps between PCs. This runner builds nothing, and has nothing of this
# project's to run but these scripts and the installers it was handed. It takes away any copy of the
# apps it came with, shows that every check fails, installs them, shows that every check works, runs the
# uninstall string each one recorded, and shows that every check fails again and nothing is left. It does
# that with each app where it was on the other PC, and again in a folder of a different name, which only
# works if everything that names the old place was rewritten. See
# crates/winappmigrator-installer/tests/elevated_real_apps.rs.
#
# Run only by the job that downloads those installers, on a runner that is thrown away afterward.
param([Parameter(Mandatory)][string]$Folder)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'real-app-checks.ps1')
Require-ThrowawayRunner

$script:failures = [Collections.Generic.List[string]]::new()
$script:table = [Collections.Generic.List[string]]::new()

function Check([string]$Label, [bool]$Passed, [string]$Detail = '') {
    if ($Passed) {
        Write-Host "  ok: $Label"
    } else {
        Write-Host "  FAILED: $Label`: $Detail"
        $script:failures.Add("$Label`: $Detail")
    }
}

function Section([string]$Title) { Write-Host "`n=== $Title" }

function Ask-Every-App($Apps, [hashtable]$RootOf, [bool]$ShouldWork, [string]$When) {
    foreach ($app in $Apps) {
        foreach ($check in $script:Checks[$app.id]) {
            $problem = Check-Result $check $RootOf[$app.id]
            $worked = -not $problem
            $script:table.Add(("{0,-22} {1,-16} {2,-8} {3}" -f $When, $app.id, $(if ($worked) { 'works' } else { 'fails' }), $check.Name))
            if ($ShouldWork) {
                Check "$($app.id), $When`: $($check.Name)" $worked $problem
            } else {
                Check "$($app.id), $When, it fails as it should: $($check.Name)" (-not $worked) 'it worked with the app not there'
                if (-not $worked) { Write-Host "      ($problem)" }
            }
        }
    }
}

$expected = Get-Content -Raw -LiteralPath (Join-Path $Folder 'expected.json') | ConvertFrom-Json
$apps = @($expected.apps)
$versionKey = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
$build = [int]$versionKey.CurrentBuildNumber
Write-Host "This PC: $($versionKey.ProductName), build $build.$($versionKey.UBR), $script:Architecture"
Write-Host "Built on: build $($expected.source_build)"

Section 'What the publishers'' own installers gave on the other PC'
$baselineFile = Join-Path $Folder 'baseline.json'
if (Test-Path -LiteralPath $baselineFile) { Get-Content -Raw -LiteralPath $baselineFile | Write-Host }

Section 'Take away any copy this runner came with'
foreach ($app in $apps) { Remove-Preinstalled $app }
$manifests = Join-Path $Folder 'manifests'
$cleanState = Get-MachineState $manifests $apps

$rounds = @(
    @{ Name = 'where it was'; RootOf = { param($app) $app.source_root }; Arguments = { param($app) @() } },
    @{ Name = 'in another folder'; RootOf = { param($app) "C:\Program Files\WAM Moved $($app.id)" }; Arguments = { param($app) @('--target-root', "C:\Program Files\WAM Moved $($app.id)") } }
)

foreach ($round in $rounds) {
    $roots = @{}
    foreach ($app in $apps) { $roots[$app.id] = [string](& $round.RootOf $app) }

    Section "$($round.Name): with the apps not here, every check fails"
    Ask-Every-App $apps $roots $false "$($round.Name), before"

    Section "$($round.Name): install them all"
    $installs = @{}
    foreach ($app in $apps) {
        $installer = Join-Path $Folder $app.installer
        $arguments = @('--apply') + @(& $round.Arguments $app)
        Write-Host "`n--- $($app.id): $installer $($arguments -join ' ')"
        $run = Run-Program $installer $arguments
        Write-Host "exit $($run.Code)"
        Write-Host $run.Output
        Check "$($app.id): the installer exits with 0" ($run.Code -eq 0) $run.Output
        Check "$($app.id): the package's hashes are right" ($run.Output -match 'Package hashes valid: true') ''
        Check "$($app.id): it read back what it installed" ($run.Output -match 'Reinstall hashes valid: true') ''
        Check "$($app.id): no errors" ($run.Output -match '(?m)^Errors: 0\s*$') ''
        if ($build -lt $expected.source_build) {
            Check "$($app.id): it says this Windows is older than the one the app came from" ($run.Output -match 'This app was on Windows build') ''
        } else {
            Check "$($app.id): it has nothing to say about this Windows being older" ($run.Output -notmatch 'This app was on Windows build') ''
        }
        if ($script:Architecture -eq 'ARM64') {
            Check "$($app.id): it says what would run through an emulator" ($run.Output -match 'emulator') ''
        } else {
            Check "$($app.id): it has nothing to say about an emulator" ($run.Output -notmatch 'emulator') ''
        }
        if ($run.Output -match '(?m)^Install id: (.+?)\s*$') { $installs[$app.id] = $Matches[1] }
        # What Settings would run, which only this install's uninstaller can do anything with.
        foreach ($entry in @(Uninstall-Entries $app.name)) {
            $says = (Get-ItemProperty -LiteralPath "HKLM:\$($entry.Key)").WindowsInstaller
            Check "$($app.id): its entry does not say Windows Installer has it" ($says -ne 1) "$says"
            $own = "`"C:\ProgramData\WinAppMigrator\installs\$($installs[$app.id])\uninstall.exe`" --uninstall $($installs[$app.id])"
            Check "$($app.id): its entry's uninstall string is this install's" ($entry.Uninstall -ceq $own) $entry.Uninstall
            Check "$($app.id): and so is its quiet one" ($entry.QuietUninstall -ceq "$own --quiet") $entry.QuietUninstall
        }
        # What the publisher's own installer left in its folder, whatever the scan made of it.
        foreach ($file in @($app.files)) {
            $path = Join-Path $roots[$app.id] $file.relative
            # It names who ran the setup, and lists where things were on that PC. See SECURITY.md.
            if ($file.relative -match '^unins\d{3}\.dat$') {
                Check "$($app.id): its Inno Setup log stayed behind" (-not (Test-Path -LiteralPath $path)) $path
                continue
            }
            if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
                Check "$($app.id): $($file.relative) is here" $false 'it is not'
            } elseif ((Sha256-Of $path) -ne $file.sha256) {
                Check "$($app.id): $($file.relative) is as it was" $false "$(Sha256-Of $path), expected $($file.sha256)"
            }
        }
        Check "$($app.id): all $(@($app.files).Count) files of its folder were looked at, by .NET" (@($app.files).Count -gt 0) ''
        # PowerShell's names go by no case, so this must not be called anything like the folder of installers.
        foreach ($emptyOne in @($app.empty_folders)) {
            $there = Test-Path -LiteralPath (Join-Path $roots[$app.id] $emptyOne) -PathType Container
            Write-Host "  note: the empty folder $emptyOne is $(if ($there) { 'here' } else { 'NOT here' })"
        }
        $status = Get-Service -ErrorAction SilentlyContinue | Where-Object { $_.Name -in 'mosquitto', 'windows_exporter' } | ForEach-Object { "$($_.Name)=$($_.Status)/$($_.StartType)" }
        Write-Host "services now: $($status -join ', ')"
    }

    Section "$($round.Name): with them installed, every check works"
    Ask-Every-App $apps $roots $true "$($round.Name), installed"
    foreach ($app in $apps) { Stop-ProcessesFrom $roots[$app.id] }

    Section "$($round.Name): as Settings would remove them, with their services running"
    foreach ($name in 'mosquitto', 'windows_exporter') {
        Start-Service -Name $name -ErrorAction SilentlyContinue
        Write-Host "  $name is $((Get-Service -Name $name -ErrorAction SilentlyContinue).Status)"
    }
    foreach ($app in $apps) {
        $entries = @(Uninstall-Entries $app.name)
        if ($entries.Count -ne 1) {
            Check "$($app.id): one entry to uninstall by" $false "$($entries.Count)"
            continue
        }
        Write-Host "`n--- $($app.id): the recorded string: $($entries[0].QuietUninstall)"
        if (-not $entries[0].QuietUninstall) {
            Check "$($app.id): it recorded a quiet uninstall string" $false $entries[0].Uninstall
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

    Section "$($round.Name): with them removed, every check fails again"
    Ask-Every-App $apps $roots $false "$($round.Name), removed"

    Section "$($round.Name): nothing is left"
    foreach ($app in $apps) {
        Check "$($app.id): its folder is gone" (-not (Test-Path -LiteralPath $roots[$app.id])) $roots[$app.id]
    }
    $differences = @(State-Differences $cleanState (Get-MachineState $manifests $apps))
    foreach ($difference in $differences) { Write-Host "  $difference" }
    Check 'the machine is as it was before any of them' ($differences.Count -eq 0) ($differences -join '; ')
}

Section 'Every check, each way'
$script:table | ForEach-Object { Write-Host $_ }

Write-Host "`nSummary: build $build, $script:Architecture, $($script:failures.Count) failures."
if ($script:failures.Count -gt 0) {
    $script:failures | ForEach-Object { Write-Host "  $_" }
    exit 1
}
# Or the job goes by the last program's exit code.
exit 0
