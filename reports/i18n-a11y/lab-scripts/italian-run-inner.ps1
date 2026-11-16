# Runs as WamItaliano, whose Windows speaks Italian: each installer installs, its result is looked at, it is uninstalled,
# and then one install is made to fail part-way, so that what it did is put back with Windows' tools answering in Italian.
param([Parameter(Mandatory)][string]$Exporter, [Parameter(Mandatory)][string]$Putty, [Parameter(Mandatory)][string]$Mosquitto)
$ErrorActionPreference = 'Continue'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
function Say([string]$Line) { "[$((Get-Date).ToString('HH:mm:ss'))] $Line" }
function Run-Installer([string]$Installer, [string[]]$Arguments) {
    Write-Host (Say "run: $Installer $($Arguments -join ' ')")
    # A windowed program: started with its output read, and waited for, as CI's run-program.ps1 does.
    $info = New-Object Diagnostics.ProcessStartInfo $Installer
    $info.Arguments = ($Arguments | ForEach-Object { if ($_ -match '\s') { '"' + $_ + '"' } else { $_ } }) -join ' '
    $info.UseShellExecute = $false
    $info.RedirectStandardOutput = $true
    $info.RedirectStandardError = $true
    $info.StandardOutputEncoding = [Text.Encoding]::UTF8
    $info.StandardErrorEncoding = [Text.Encoding]::UTF8
    $process = [Diagnostics.Process]::Start($info)
    $readOut = $process.StandardOutput.ReadToEndAsync()
    $readErr = $process.StandardError.ReadToEndAsync()
    $process.WaitForExit()
    $exit = $process.ExitCode
    $output = @(($readOut.Result + $readErr.Result) -split "`r?`n")
    $output | ForEach-Object { Write-Host "    $_" }
    Write-Host (Say "exit $exit")
    $id = ($output | Where-Object { "$_" -match '^Install id:\s*(\S+)' } | Select-Object -First 1)
    if ($id -and "$id" -match '^Install id:\s*(\S+)') { $id = $Matches[1] } else { $id = $null }
    return @{ Exit = $exit; Id = $id }
}
function Show-State([string]$When) {
    Say "--- $When"
    foreach ($name in 'windows_exporter', 'mosquitto') {
        $service = Get-Service -Name $name -ErrorAction SilentlyContinue
        "    service $name`: $(if ($service) { "$($service.Status), $($service.StartType)" } else { 'none' })"
    }
    $rules = @(Get-NetFirewallRule -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -like '*windows_exporter*' })
    "    firewall rules for windows_exporter: $($rules.Count)"
    $tasks = @(Get-ScheduledTask -TaskPath '\WAM Lab\' -ErrorAction SilentlyContinue)
    "    tasks in \WAM Lab\: $($tasks.Count)"
    $links = @(Get-ChildItem 'C:\ProgramData\Microsoft\Windows\Start Menu\Programs' -Recurse -Filter 'PuTTY*.lnk' -ErrorAction SilentlyContinue)
    "    PuTTY shortcuts: $($links.Count)"
    foreach ($folder in 'C:\Program Files\windows_exporter', 'C:\Program Files\mosquitto', 'C:\Program Files\PuTTY') {
        "    $folder`: $(if (Test-Path -LiteralPath $folder) { "$(@(Get-ChildItem -LiteralPath $folder -Recurse -File).Count) files" } else { 'none' })"
    }
}

Say "user $(whoami), display language $((Get-UICulture).Name), region $((Get-Culture).Name), elevated: $(([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole('Administrators'))"
Say "sc.exe says: $((& sc.exe query WamNoSuchService) -join ' ')"
Show-State 'before'

$installs = @{}
foreach ($pair in @(@('windows_exporter', $Exporter), @('mosquitto', $Mosquitto), @('putty', $Putty))) {
    $result = Run-Installer $pair[1] @('--apply')
    $installs[$pair[0]] = $result
}
Show-State 'after installing all three'
foreach ($name in 'windows_exporter', 'mosquitto') {
    $state = (& sc.exe query $name | Out-String)
    Say "sc.exe query $name`:"
    $state.Trim() -split "`r?`n" | ForEach-Object { "    $_" }
}

foreach ($pair in @(@('windows_exporter', $Exporter), @('mosquitto', $Mosquitto), @('putty', $Putty))) {
    $id = $installs[$pair[0]].Id
    if (-not $id) { Say "no install id for $($pair[0]), so it cannot be uninstalled"; continue }
    $null = Run-Installer $pair[1] @('--uninstall', $id, '--quiet')
}
Show-State 'after uninstalling all three'

Say '=== an install made to fail part-way: a file it must replace is held open'
$target = 'C:\Program Files\windows_exporter'
New-Item -ItemType Directory -Force -Path $target | Out-Null
$held = Join-Path $target 'windows_exporter.exe'
Set-Content -LiteralPath $held -Value 'held open, so it cannot be replaced'
$lock = [IO.File]::Open($held, 'Open', 'ReadWrite', 'None')
try {
    $result = Run-Installer $Exporter @('--apply', '--force')
} finally {
    $lock.Dispose()
}
Show-State 'after the install that failed'
Say "the held file says: $(Get-Content -LiteralPath $held -Raw)"
Remove-Item -LiteralPath $target -Recurse -Force -ErrorAction SilentlyContinue
Say 'done'
