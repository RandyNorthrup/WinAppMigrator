# testpc1: a local administrator whose Windows speaks Italian, for running WinAppMigrator where Windows' own tools
# answer in Italian. Technician's language is not changed. Lab use only.
$ErrorActionPreference = 'Stop'
if ($env:COMPUTERNAME -ne 'PATIO01') { throw 'testpc1 only' }
$name = 'WamItaliano'
$secretFile = 'C:\WamRepro\italiano-password.txt'
if (-not (Get-LocalUser -Name $name -ErrorAction SilentlyContinue)) {
    $chars = [char[]]'abcdefghijkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789'
    $plain = -join (1..20 | ForEach-Object { $chars | Get-Random })
    New-LocalUser -Name $name -Password (ConvertTo-SecureString $plain -AsPlainText -Force) -FullName 'WinAppMigrator Italian test' -PasswordNeverExpires | Out-Null
    Add-LocalGroupMember -SID 'S-1-5-32-544' -Member $name
    Set-Content -LiteralPath $secretFile -Value $plain -NoNewline
    & icacls.exe $secretFile /inheritance:r /grant:r '*S-1-5-32-544:(F)' '*S-1-5-18:(F)' | Out-Null
    'created'
}
$plain = Get-Content -LiteralPath $secretFile -Raw

function Invoke-AsItaliano([string] $Script, [string] $Log) {
    $task = 'WamItaliano-Run'
    Unregister-ScheduledTask -TaskName $task -Confirm:$false -ErrorAction SilentlyContinue
    $action = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument "-NoProfile -ExecutionPolicy Bypass -Command `"& { $Script } *> '$Log'`""
    Register-ScheduledTask -TaskName $task -Action $action -User "$env:COMPUTERNAME\$name" -Password $plain -RunLevel Highest | Out-Null
    Start-ScheduledTask -TaskName $task
    Start-Sleep -Seconds 2
    $deadline = (Get-Date).AddMinutes(10)
    while ((Get-ScheduledTask -TaskName $task).State -eq 'Running' -and (Get-Date) -lt $deadline) { Start-Sleep -Seconds 1 }
    Unregister-ScheduledTask -TaskName $task -Confirm:$false
    Get-Content -LiteralPath $Log -Encoding utf8
}

# Its first run makes its profile; this one sets its languages there.
Invoke-AsItaliano 'Set-WinUILanguageOverride -Language it-IT; Set-WinUserLanguageList -LanguageList it-IT -Force; Set-Culture it-IT; Set-WinHomeLocation -GeoId 118; "set"' 'C:\WamRepro\logs\italiano-set.log'
# A new logon picks them up.
Invoke-AsItaliano '(Get-WinUILanguageOverride).Name; (Get-Culture).Name; & sc.exe query WinAppMigratorNoSuchService; & netsh.exe advfirewall firewall show rule name=WinAppMigratorNoSuchRule; & schtasks.exe /query /tn \WinAppMigratorNoSuchTask; & whoami.exe /groups /fo csv | Select-Object -First 3' 'C:\WamRepro\logs\italiano-check.log'
