param(
    [string] $GuiExe = ".\target\release\winappmigrator-gui.exe",
    [int] $StartupTimeoutSeconds = 20
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName Microsoft.VisualBasic

$tool = Resolve-Path -LiteralPath $GuiExe
$process = Start-Process -FilePath $tool -PassThru -WindowStyle Normal

try {
    $deadline = (Get-Date).AddSeconds($StartupTimeoutSeconds)
    do {
        Start-Sleep -Milliseconds 250
        $process.Refresh()
    } while ($process.MainWindowHandle -eq 0 -and (Get-Date) -lt $deadline)

    if ($process.MainWindowHandle -eq 0) {
        throw "GUI window did not appear within $StartupTimeoutSeconds seconds."
    }

    [Microsoft.VisualBasic.Interaction]::AppActivate($process.Id) | Out-Null
    Start-Sleep -Milliseconds 500
    [System.Windows.Forms.SendKeys]::SendWait("{TAB}")
    Start-Sleep -Milliseconds 100
    [System.Windows.Forms.SendKeys]::SendWait("{ENTER}")
    Start-Sleep -Milliseconds 500

    if ($process.HasExited) {
        throw "GUI exited during click-through smoke."
    }

    Write-Host "GUI click-through smoke passed for process id $($process.Id)."
}
finally {
    if (-not $process.HasExited) {
        [Microsoft.VisualBasic.Interaction]::AppActivate($process.Id) | Out-Null
        [System.Windows.Forms.SendKeys]::SendWait("%{F4}")
        Start-Sleep -Seconds 1
        if (-not $process.HasExited) {
            Stop-Process -Id $process.Id -Force
        }
    }
}
