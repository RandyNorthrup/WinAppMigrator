$ErrorActionPreference = 'Continue'
$dl = 'C:\WamRepro\dl\realapps'
$p = Start-Process msiexec.exe -ArgumentList @('/x', "`"$dl\windows_exporter-0.32.1-amd64.msi`"", '/qn', '/norestart') -Wait -PassThru; "exporter removal: $($p.ExitCode)"
$p = Start-Process msiexec.exe -ArgumentList @('/x', "`"$dl\putty-64bit-0.85-installer.msi`"", '/qn', '/norestart') -Wait -PassThru; "putty removal: $($p.ExitCode)"
$uninstaller = 'C:\Program Files\mosquitto\Uninstall.exe'
if (Test-Path $uninstaller) { $p = Start-Process $uninstaller -ArgumentList '/S' -Wait -PassThru; "mosquitto removal: $($p.ExitCode)"; Start-Sleep -Seconds 5 }
Unregister-ScheduledTask -TaskPath '\WAM Lab\' -TaskName 'windows_exporter version' -Confirm:$false -ErrorAction SilentlyContinue
Get-Service windows_exporter, mosquitto -ErrorAction SilentlyContinue | Format-Table Name, Status -AutoSize | Out-String
"firewall rules: $(@(Get-NetFirewallRule | Where-Object { $_.DisplayName -like '*windows_exporter*' }).Count)"
foreach ($f in 'C:\Program Files\windows_exporter', 'C:\Program Files\mosquitto', 'C:\Program Files\PuTTY') { "$f exists: $(Test-Path $f)" }
