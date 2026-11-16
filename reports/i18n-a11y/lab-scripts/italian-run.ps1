# Runs italian-run-inner.ps1 as WamItaliano, whose Windows speaks Italian, elevated, in a scheduled task, and prints its log.
param([Parameter(Mandatory)][string]$Exporter, [Parameter(Mandatory)][string]$Putty, [Parameter(Mandatory)][string]$Mosquitto)
$ErrorActionPreference = 'Stop'
$plain = Get-Content -LiteralPath 'C:\WamRepro\italiano-password.txt' -Raw
$task = 'WamItaliano-Run'
$log = 'C:\WamRepro\logs\italian-run.log'
Remove-Item -LiteralPath $log -ErrorAction SilentlyContinue
$inner = 'C:\WamRepro\scripts\italian-run-inner.ps1'
$command = "& '$inner' -Exporter '$Exporter' -Putty '$Putty' -Mosquitto '$Mosquitto' *> '$log'"
$action = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument "-NoProfile -ExecutionPolicy Bypass -Command `"$command`""
Unregister-ScheduledTask -TaskName $task -Confirm:$false -ErrorAction SilentlyContinue
Register-ScheduledTask -TaskName $task -Action $action -User "$env:COMPUTERNAME\WamItaliano" -Password $plain -RunLevel Highest | Out-Null
Start-ScheduledTask -TaskName $task
Start-Sleep -Seconds 3
$deadline = (Get-Date).AddMinutes(45)
while ((Get-ScheduledTask -TaskName $task).State -eq 'Running' -and (Get-Date) -lt $deadline) { Start-Sleep -Seconds 5 }
"task: $((Get-ScheduledTask -TaskName $task).State), last result $((Get-ScheduledTaskInfo -TaskName $task).LastTaskResult)"
Unregister-ScheduledTask -TaskName $task -Confirm:$false
Get-Content -LiteralPath $log -Encoding utf8
