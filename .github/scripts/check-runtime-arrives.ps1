# What a PC with none of the 64-bit Visual C++ 2015+ runtime does with an installer built the way the app
# builds one. Every runner has that runtime, so nothing else shows this. Other versions and the 32-bit one
# stay, as the broker does not use them. elevated_real_apps.rs has built Mosquitto with the
# runtime in it, from the publisher's copy on this runner. This shows the broker working, takes the
# runtime off the runner with its own uninstallers and then whatever of it is left, shows the broker
# cannot start, removes the publisher's copy, installs what was built, and shows that the runtime came
# back from what was in the package, and that the broker serves a real client.
#
# Our own programs link the C runtime statically, so taking it away does not stop them. It can stop other
# things on this runner, which is why this is the last thing that happens on it.
#
# Run only by the job that built that installer, on a runner that is thrown away afterward.
param([Parameter(Mandatory)][string]$Folder)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'real-app-checks.ps1')
Require-ThrowawayRunner

$script:failures = [Collections.Generic.List[string]]::new()
# STATUS_DLL_NOT_FOUND, as what a program that cannot load one exits with.
$DllNotFound = -1073741515
$RuntimeFiles = 'vcruntime140.dll', 'vcruntime140_1.dll', 'msvcp140.dll', 'msvcp140_1.dll', 'msvcp140_2.dll', 'concrt140.dll', 'vccorlib140.dll'
$RuntimeKeys = 'SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64', 'SOFTWARE\WOW6432Node\Microsoft\VisualStudio\14.0\VC\Runtimes\x64'

function Check([string]$Label, [bool]$Passed, [string]$Detail = '') {
    if ($Passed) {
        Write-Host "  ok: $Label"
    } else {
        Write-Host "  FAILED: $Label`: $Detail"
        $script:failures.Add("$Label`: $Detail")
    }
}

# Windows asks a person what to do about a program that cannot load a DLL, in a window that keeps the
# program waiting. A child takes this setting from whoever starts it, and with it there is no window.
Add-Type -TypeDefinition @'
using System.Runtime.InteropServices;
public static class WamErrorMode {
    [DllImport("kernel32.dll")]
    public static extern uint SetErrorMode(uint mode);
}
'@
[WamErrorMode]::SetErrorMode(0x8001) | Out-Null

# A clean PC has none of the tools this runner has on its PATH, several of which carry a copy of this
# runtime, so a program starts as it would there: with Windows' own folders to look in, and in one of those.
function Start-As-On-A-Clean-PC([string]$Program, [string]$Arguments) {
    $info = [Diagnostics.ProcessStartInfo]::new($Program, $Arguments)
    $info.UseShellExecute = $false
    $info.CreateNoWindow = $true
    $info.WorkingDirectory = $env:SystemRoot
    $info.Environment['PATH'] = "$env:SystemRoot\System32;$env:SystemRoot"
    return [Diagnostics.Process]::Start($info)
}

# What a program does when it is asked only to say how it is used.
function Exit-Code-Of([string]$Program, [string]$Arguments) {
    $process = Start-As-On-A-Clean-PC $Program $Arguments
    if (-not $process.WaitForExit(30000)) {
        $process.Kill()
        return 'it was still running after 30 seconds'
    }
    return $process.ExitCode
}

function Runtime-State {
    foreach ($entry in @(Uninstall-Entries 'Microsoft Visual C++')) {
        if ($entry.DisplayName -match '(2015|2017|2019|2022).*(x64)') { Write-Host "  in Settings: $($entry.DisplayName): $($entry.Uninstall)" }
    }
    foreach ($file in $RuntimeFiles) {
        $path = "$env:SystemRoot\System32\$file"
        if (Test-Path -LiteralPath $path) {
            $item = Get-Item -LiteralPath $path
            Write-Host "  $path, $($item.VersionInfo.FileVersion), $((Get-AuthenticodeSignature -LiteralPath $path).Status)"
        } else {
            Write-Host "  $path is not there"
        }
    }
    foreach ($key in $RuntimeKeys) {
        $found = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey($key)
        Write-Host "  HKLM\$key Installed: $(if ($found) { $found.GetValue('Installed') } else { '(no key)' })"
    }
}

$expected = Get-Content -Raw -LiteralPath (Join-Path $Folder 'expected.json') | ConvertFrom-Json
$app = (Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'real-apps.json') | ConvertFrom-Json).apps | Where-Object id -eq 'mosquitto'
$root = $app.installed_at
$broker = Join-Path $root 'mosquitto.exe'

Write-Host "`n=== With the runtime this runner came with"
Runtime-State
Stop-Service -Name mosquitto -Force -ErrorAction SilentlyContinue
$before = Exit-Code-Of $broker '--help'
Write-Host "  the publisher's broker, asked how it is used, exits with $before"
Check 'the publisher''s broker can start' ($before -ne $DllNotFound) "$before"

Write-Host "`n=== Take the runtime away, by its own uninstallers"
foreach ($entry in @(Uninstall-Entries 'Microsoft Visual C++')) {
    if ($entry.DisplayName -notmatch '(2015|2017|2019|2022).*(x64)') { continue }
    Write-Host "  $($entry.DisplayName): $($entry.Uninstall)"
    if ($entry.Uninstall -match '(?i)msiexec.*(\{[0-9a-f-]+\})') {
        # Visual Studio depends on these here, and without this, Windows Installer says it has done
        # as it was asked and leaves them.
        $process = Start-Process msiexec.exe -ArgumentList '/x', $Matches[1], '/qn', '/norestart', 'IGNOREDEPENDENCIES=ALL' -PassThru
    } else {
        $command = Split-CommandLine $entry.Uninstall
        $process = Start-Process $command.Program -ArgumentList (@($command.Arguments) + '/quiet', '/norestart') -PassThru
    }
    if ($process.WaitForExit(300000)) {
        Write-Host "    it exited with $($process.ExitCode)"
    } else {
        Check "$($entry.DisplayName)'s uninstaller finishes" $false 'it was still going after five minutes, and was stopped'
        $process.Kill()
        $process.WaitForExit(30000) | Out-Null
    }
}
Runtime-State

Write-Host "`n=== And what is left of it, which other things on this runner hold on to"
foreach ($file in $RuntimeFiles) {
    $path = "$env:SystemRoot\System32\$file"
    if (-not (Test-Path -LiteralPath $path)) { continue }
    # A DLL something has loaded can be renamed, and nothing new will find it.
    & takeown.exe /f $path /a | Out-Null
    & icacls.exe $path /grant 'Administrators:F' | Out-Null
    Rename-Item -LiteralPath $path -NewName "$file.taken-away"
    Write-Host "  $path is out of the way"
}
foreach ($key in $RuntimeKeys) {
    [Microsoft.Win32.Registry]::LocalMachine.DeleteSubKeyTree($key, $false)
}
Runtime-State

$without = Exit-Code-Of $broker '--help'
Write-Host "  the publisher's broker, with the runtime gone, exits with $without"
if ($without -ne $DllNotFound) {
    # Then nothing that follows would show anything, and it says so instead.
    Write-Host "`nThe runtime could not be taken away from this runner: the broker still exits with $without."
    exit 1
}
Check 'with the runtime gone, the broker cannot start' $true

Write-Host "`n=== Take the publisher's copy away"
Remove-Preinstalled $app
Check 'its folder is gone' (-not (Test-Path -LiteralPath $root)) $root

Write-Host "`n=== Install what was built"
$log = Join-Path $Folder 'install.log'
$run = Run-Program (Join-Path $Folder $expected.installer) @('--apply', '--log', $log)
Write-Host "exit $($run.Code)"
Write-Host $run.Output
Check 'the installer exits with 0' ($run.Code -eq 0) $run.Output
Check 'no errors' ($run.Output -match '(?m)^Errors: 0\s*$') ''
Check 'it read back what it installed' ($run.Output -match 'Reinstall hashes valid: true') ''
$said = @(Get-Content -LiteralPath $log | Where-Object { $_ -match '(?i)prerequisite' })
Write-Host 'What its log says of the runtime:'
$said | ForEach-Object { Write-Host "  $_" }
Check "its log says it installed $($expected.prerequisite)" (@($said | Where-Object { $_ -match 'installed prerequisite' }).Count -gt 0) ($said -join '; ')

Write-Host "`n=== The runtime is back, from what was in the package"
Runtime-State
foreach ($file in 'vcruntime140.dll', 'msvcp140.dll') {
    $path = "$env:SystemRoot\System32\$file"
    Check "$file is back" (Test-Path -LiteralPath $path) $path
    if (Test-Path -LiteralPath $path) {
        $signature = Get-AuthenticodeSignature -LiteralPath $path
        Check "$file is Microsoft's" ($signature.Status -eq 'Valid' -and $signature.SignerCertificate.Subject -match 'O=Microsoft Corporation') "$($signature.Status) $($signature.SignerCertificate.Subject)"
    }
}
Check 'Windows lists it' (@(Uninstall-Entries 'Microsoft Visual C++' | Where-Object { $_.DisplayName -match '(2015|2017|2019|2022).*(x64)' }).Count -gt 0) ''
$key = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey($RuntimeKeys[0])
Check 'its record says it is there' ($key -and $key.GetValue('Installed') -eq 1) ''

Write-Host "`n=== The broker runs on it"
$after = Exit-Code-Of $broker '--help'
Write-Host "  the broker, asked how it is used, exits with $after"
Check 'the installed broker starts, and says what it said before' ($after -eq $before) "$after, and it was $before"
$port = Free-Port
$broker_process = Start-As-On-A-Clean-PC $broker "-p $port"
try {
    Start-Sleep -Seconds 3
    Check 'as on a clean PC, it stays up' (-not $broker_process.HasExited) "exit $(if ($broker_process.HasExited) { $broker_process.ExitCode })"
    $problem = Mqtt-Problem $port
    Check 'as on a clean PC, it answers a real MQTT client' (-not $problem) $problem
} finally {
    if (-not $broker_process.HasExited) { $broker_process.Kill() }
}
$problem = Service-Problem 'mosquitto' $root { Mqtt-Problem 1883 }
Check 'its service starts, stays up, answers a real MQTT client, and stops' (-not $problem) $problem

Write-Host "`n=== An uninstall leaves the runtime, as other programs may use it"
$entries = @(Uninstall-Entries $app.name)
if ($entries.Count -eq 1) {
    $command = Split-CommandLine $entries[0].QuietUninstall
    $run = Run-Program $command.Program $command.Arguments
    Write-Host "exit $($run.Code)"
    Write-Host $run.Output
    Check 'its uninstall string works' ($run.Code -eq 0) $run.Output
    Check 'the broker is gone' (Wait-Until { -not (Test-Path -LiteralPath $root) } 60) $root
    Check 'the runtime is not' (Test-Path -LiteralPath "$env:SystemRoot\System32\vcruntime140.dll") ''
} else {
    Check 'one entry to uninstall by' $false "$($entries.Count)"
}

Write-Host "`nSummary: $($script:failures.Count) failures."
if ($script:failures.Count -gt 0) {
    $script:failures | ForEach-Object { Write-Host "  $_" }
    exit 1
}
exit 0
