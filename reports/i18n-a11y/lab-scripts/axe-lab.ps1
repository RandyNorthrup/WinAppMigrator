param([string]$App = 'C:\WamRepro\dl\WinAppMigrator-v0.7.0-pre.exe', [string]$Stub = 'C:\WamRepro\dl\winappmigrator-installer-0.7.0-pre.exe', [string]$Axe = 'C:\WamRepro\axe\AxeWindowsCLI.exe', [string]$Out = 'C:\WamRepro\logs\axe')
New-Item -ItemType Directory -Force -Path $Out | Out-Null
function Scan([string]$Name, [string]$Program, [string[]]$Arguments, [int]$Settle) {
    $starting = @{ FilePath = $Program; PassThru = $true }
    if ($Arguments) { $starting.ArgumentList = $Arguments }
    $process = Start-Process @starting
    Start-Sleep -Seconds $Settle
    "== $Name (pid $($process.Id), running: $(-not $process.HasExited))"
    & $Axe --processid $process.Id --outputdirectory $Out --scanid $Name --alwayssavetestfile 2>&1 | ForEach-Object { "  $_" }
    "  exit $LASTEXITCODE"
    if (-not $process.HasExited) { Stop-Process -Id $process.Id -Force }
    Start-Sleep -Seconds 2
}
Get-Process -Name 'WinAppMigrator-*', 'winappmigrator-installer*' -ErrorAction SilentlyContinue | Stop-Process -Force
Scan 'app' $App @() 25
Scan 'uninstaller-no-record' $Stub @('--uninstall', 'no-such-install') 10
Scan 'stub-no-package' $Stub @() 10
Get-ChildItem $Out | Select-Object Name, Length | Format-Table -AutoSize | Out-String
