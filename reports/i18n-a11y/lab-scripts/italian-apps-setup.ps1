# testpc1: install the apps the Italian-Windows check moves, as their own installers would, from the URLs and hashes the
# project's CI pins (.github/scripts/real-apps.json). windows_exporter brings a service and a firewall rule, Mosquitto a
# service, PuTTY shortcuts; a scheduled task is added that runs windows_exporter, as an app's own task would.
$ErrorActionPreference = 'Stop'
if ($env:COMPUTERNAME -ne 'PATIO01') { throw 'testpc1 only' }
$downloads = 'C:\WamRepro\dl\realapps'
New-Item -ItemType Directory -Force -Path $downloads | Out-Null
$apps = @(
    @{ File = 'windows_exporter-0.32.1-amd64.msi'; Url = 'https://github.com/prometheus-community/windows_exporter/releases/download/v0.32.1/windows_exporter-0.32.1-amd64.msi'; Sha = '8E1ADE6A2FBCDEB4774C300B598667CD7888246BE8FCC4B586D11B3680433981' },
    @{ File = 'mosquitto-2.1.2-install-windows-x64.exe'; Url = 'https://mosquitto.org/files/binary/win64/mosquitto-2.1.2-install-windows-x64.exe'; Sha = '58008AD7A22ADA0B4073AFA415801746E027C5F583E4FA52D0F4E9193B98D6AA' },
    @{ File = 'putty-64bit-0.85-installer.msi'; Url = 'https://the.earth.li/~sgtatham/putty/0.85/w64/putty-64bit-0.85-installer.msi'; Sha = '2213EBACF962B709411D654A40087508A694756BC4E41C5A1C24AD86596B2511' }
)
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
foreach ($app in $apps) {
    $path = Join-Path $downloads $app.File
    if (-not (Test-Path -LiteralPath $path)) {
        Invoke-WebRequest -Uri $app.Url -OutFile $path -UseBasicParsing
    }
    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $path).Hash
    if ($hash -ne $app.Sha) { Remove-Item -LiteralPath $path; throw "$($app.File) is not the file it should be: $hash" }
    "$($app.File): hash as pinned"
}

$exporter = Join-Path $downloads 'windows_exporter-0.32.1-amd64.msi'
$process = Start-Process msiexec.exe -ArgumentList @('/i', "`"$exporter`"", '/qn', '/norestart', 'ADD_FIREWALL_EXCEPTION=yes', 'ADDLOCAL=FirewallException') -Wait -PassThru
"windows_exporter: msiexec exit $($process.ExitCode)"
$process = Start-Process (Join-Path $downloads 'mosquitto-2.1.2-install-windows-x64.exe') -ArgumentList '/S' -Wait -PassThru
"mosquitto: exit $($process.ExitCode)"
$putty = Join-Path $downloads 'putty-64bit-0.85-installer.msi'
$process = Start-Process msiexec.exe -ArgumentList @('/i', "`"$putty`"", '/qn', '/norestart') -Wait -PassThru
"putty: msiexec exit $($process.ExitCode)"

$exe = 'C:\Program Files\windows_exporter\windows_exporter.exe'
$action = New-ScheduledTaskAction -Execute $exe -Argument '--version'
$trigger = New-ScheduledTaskTrigger -Daily -At '03:00'
Register-ScheduledTask -TaskPath '\WAM Lab\' -TaskName 'windows_exporter version' -Action $action -Trigger $trigger -User 'SYSTEM' -Force | Out-Null

'--- what is there now'
Get-Service windows_exporter, mosquitto -ErrorAction SilentlyContinue | Format-Table Name, Status, StartType -AutoSize | Out-String
Get-NetFirewallRule -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -like '*windows_exporter*' -or $_.DisplayName -like '*exporter*' } | Format-Table DisplayName, Direction, Action, Enabled -AutoSize | Out-String
Get-ScheduledTask -TaskPath '\WAM Lab\' | Format-Table TaskPath, TaskName -AutoSize | Out-String
Get-ChildItem 'C:\ProgramData\Microsoft\Windows\Start Menu\Programs' -Recurse -Filter '*PuTTY*' | Select-Object -ExpandProperty FullName
