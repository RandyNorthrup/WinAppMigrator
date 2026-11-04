# What each real app has to do for its move to count, shared by the runner that installs it from its
# publisher and the runners it is moved to, so that all of them ask exactly the same of it. Each check
# asks Windows to use the thing, rather than looking for its bytes or its keys, and answers with what
# went wrong, or with nothing if it worked. The same check has to fail where the app is not there, or
# it proves nothing. See crates/winappmigrator-installer/tests/elevated_real_apps.rs.
#
# Run only by jobs on a runner that is thrown away afterward: this removes apps and starts services.

$script:StartMenu = 'C:\ProgramData\Microsoft\Windows\Start Menu\Programs'
$script:Architecture = (Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment').PROCESSOR_ARCHITECTURE
$script:SevenZipShellExtension = '{23170F69-40C1-278A-1000-000100020000}'

Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
using System.Text;
public static class WamAssociation {
    [DllImport("shlwapi.dll", CharSet = CharSet.Unicode)]
    static extern int AssocQueryStringW(int flags, int what, string association, string verb, StringBuilder result, ref uint size);
    // 2 is ASSOCSTR_EXECUTABLE: the program Windows would run for it.
    public static string ProgramFor(string association, string verb) {
        uint size = 1024;
        var text = new StringBuilder((int)size);
        return AssocQueryStringW(0, 2, association, verb, text, ref size) == 0 ? text.ToString() : "";
    }
}
'@

function Require-ThrowawayRunner {
    if ($env:GITHUB_ACTIONS -ne 'true' -or $env:RUNNER_ENVIRONMENT -ne 'github-hosted' -or $env:WAM_ELEVATED_E2E -ne '1') {
        throw 'this removes apps and changes the machine, so it only runs on GitHub''s hosted runner'
    }
}

function Is-Inside([string]$Path, [string]$Root) {
    if ([string]::IsNullOrEmpty($Path)) { return $false }
    return $Path.StartsWith($Root.TrimEnd('\') + '\', [StringComparison]::OrdinalIgnoreCase)
}

function New-WorkFolder {
    $folder = Join-Path $env:RUNNER_TEMP ([IO.Path]::GetRandomFileName())
    New-Item -ItemType Directory -Path $folder | Out-Null
    return $folder
}

function Sha256-Of([string]$Path) {
    $stream = [IO.File]::OpenRead($Path)
    try {
        return [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($stream)).ToLowerInvariant()
    } finally {
        $stream.Dispose()
    }
}

. (Join-Path $PSScriptRoot 'run-program.ps1')

function Start-Hidden([string]$Program, [string]$Arguments, [switch]$CaptureOutput) {
    if (-not [IO.File]::Exists($Program)) { throw "$Program is not there" }
    $info = [Diagnostics.ProcessStartInfo]::new($Program, $Arguments)
    $info.UseShellExecute = $false
    $info.CreateNoWindow = $true
    $info.RedirectStandardOutput = [bool]$CaptureOutput
    return [Diagnostics.Process]::Start($info)
}

function Wait-Until([scriptblock]$Condition, [int]$Seconds) {
    $deadline = [DateTime]::UtcNow.AddSeconds($Seconds)
    while ([DateTime]::UtcNow -lt $deadline) {
        if (& $Condition) { return $true }
        Start-Sleep -Milliseconds 250
    }
    return [bool](& $Condition)
}

function Processes-From([string]$Root) {
    return @(Get-CimInstance Win32_Process | Where-Object { Is-Inside $_.ExecutablePath $Root })
}

function Stop-ProcessesFrom([string]$Root) {
    foreach ($process in (Processes-From $Root)) {
        Stop-Process -Id $process.ProcessId -Force -ErrorAction SilentlyContinue
    }
}

# Whether a program of this name is running from the app's folder, and what it was told.
function Started-From([string]$Root, [string]$Program, [string]$Mentioning = '') {
    # What a block sets in here would be lost when it ends, unless it is set on something from out here.
    $seen = @{ Found = @() }
    $started = Wait-Until {
        $seen.Found = @(Processes-From $Root | Where-Object { $_.Name -ieq $Program -and $_.CommandLine -like "*$Mentioning*" })
        $seen.Found.Count -gt 0
    } 20
    foreach ($process in $seen.Found) { Stop-Process -Id $process.ProcessId -Force -ErrorAction SilentlyContinue }
    return $started
}

function Free-Port {
    $listener = [Net.Sockets.TcpListener]::new([Net.IPAddress]::Loopback, 0)
    $listener.Start()
    $port = $listener.LocalEndpoint.Port
    $listener.Stop()
    return $port
}

function Uninstall-Entries([string]$StartsWith) {
    $places = @(
        @([Microsoft.Win32.Registry]::LocalMachine, 'SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall'),
        @([Microsoft.Win32.Registry]::LocalMachine, 'SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall'),
        @([Microsoft.Win32.Registry]::CurrentUser, 'SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall')
    )
    foreach ($place in $places) {
        $list = $place[0].OpenSubKey($place[1])
        if (-not $list) { continue }
        foreach ($name in $list.GetSubKeyNames()) {
            $key = $list.OpenSubKey($name)
            if (-not $key) { continue }
            $display = [string]$key.GetValue('DisplayName')
            if ($display.StartsWith($StartsWith, [StringComparison]::OrdinalIgnoreCase)) {
                [pscustomobject]@{
                    Key             = "$($place[1])\$name"
                    DisplayName     = $display
                    Uninstall       = [string]$key.GetValue('UninstallString')
                    QuietUninstall  = [string]$key.GetValue('QuietUninstallString')
                    InstallLocation = [string]$key.GetValue('InstallLocation')
                    SystemComponent = $key.GetValue('SystemComponent')
                }
            }
        }
    }
}

# A command line as Windows would split it, into a program and what it is told.
function Split-CommandLine([string]$CommandLine) {
    if ($CommandLine -match '^"([^"]+)"\s*(.*)$') {
        $program = $Matches[1]; $rest = $Matches[2]
    } elseif ($CommandLine -match '^(\S+)\s*(.*)$') {
        $program = $Matches[1]; $rest = $Matches[2]
    } else {
        throw "could not read this command line: $CommandLine"
    }
    return [pscustomobject]@{
        Program   = $program
        Arguments = @($rest -split '\s+' | Where-Object { $_ })
    }
}

# With what the app recorded for it, the way its own uninstaller would be started.
function Remove-Preinstalled($App) {
    foreach ($entry in @(Uninstall-Entries $App.name)) {
        Write-Host "  this runner came with $($entry.DisplayName), at $($entry.Key): $($entry.Uninstall)"
        if ($entry.Uninstall -match '(?i)msiexec.*(\{[0-9a-f-]+\})') {
            $process = Start-Process msiexec.exe -ArgumentList '/x', $Matches[1], '/qn', '/norestart' -Wait -PassThru
            $code = $process.ExitCode
        } else {
            $command = Split-CommandLine $entry.Uninstall
            $silent = if ($entry.Uninstall -match '(?i)unins\d+\.exe') { '/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART' } else { @('/S') }
            $process = Start-Process $command.Program -ArgumentList $silent -Wait -PassThru
            $code = $process.ExitCode
        }
        Write-Host "  its uninstaller exited with $code"
        if (-not (Wait-Until { @(Uninstall-Entries $App.name).Count -eq 0 } 60)) {
            throw "$($App.name) would not uninstall"
        }
    }
    # An uninstaller can leave a file or two, and one that deletes itself takes its time. A PC that never
    # had the app has none of that, and an install would stop at it, so what is left goes too, and is said.
    # The list of apps says where each goes, and what a build hands on says where it was.
    $folder = if ($App.installed_at) { $App.installed_at } else { $App.source_root }
    if ((Test-Path -LiteralPath $folder) -and -not (Wait-Until { -not (Test-Path -LiteralPath $folder) } 60)) {
        $left = @(Get-ChildItem -LiteralPath $folder -Recurse -Force -File | ForEach-Object Name)
        Write-Host "  its uninstaller left $($left.Count) files, which go now: $($left -join ', ')"
        Remove-Item -LiteralPath $folder -Recurse -Force
    }
}

# --- What an account has in an app's keys, and what each is written as.

function Setting-Bytes($Setting) {
    return [Text.Encoding]::Unicode.GetBytes($Setting.marker)
}

function Set-AccountSetting($Setting) {
    $key = [Microsoft.Win32.Registry]::CurrentUser.CreateSubKey($Setting.key)
    if ($Setting.kind -eq 'binary-utf16') {
        $key.SetValue($Setting.name, [byte[]](Setting-Bytes $Setting), [Microsoft.Win32.RegistryValueKind]::Binary)
    } else {
        $key.SetValue($Setting.name, [string]$Setting.marker, [Microsoft.Win32.RegistryValueKind]::String)
    }
    $key.Dispose()
}

# Where a file an account has for an app is, in this account's folders.
function AccountFile-Path($File) {
    return Join-Path ([Environment]::GetFolderPath('ApplicationData')) $File.relative
}

# What is wrong with the value as this account has it now, or nothing.
function AccountSetting-Problem($Setting) {
    $key = [Microsoft.Win32.Registry]::CurrentUser.OpenSubKey($Setting.key)
    if (-not $key) { return "there is no key $($Setting.key)" }
    $value = $key.GetValue($Setting.name, $null, 'DoNotExpandEnvironmentNames')
    if ($null -eq $value) { return "there is no value $($Setting.name) in $($Setting.key)" }
    if ($Setting.kind -eq 'binary-utf16') {
        if ($key.GetValueKind($Setting.name) -ne 'Binary') { return "it is a $($key.GetValueKind($Setting.name))" }
        $expected = [Convert]::ToHexString([byte[]](Setting-Bytes $Setting))
        $got = [Convert]::ToHexString([byte[]]$value)
        if ($got -ne $expected) { return "it holds $got, and $expected was expected" }
    } elseif ($value -cne $Setting.marker) {
        return "it holds [$value]"
    }
    return ''
}

# --- Each kind of check. Nothing returned, or an empty string, means it worked.

function Entry-Problem([string]$Name) {
    $entries = @(Uninstall-Entries $Name)
    if ($entries.Count -ne 1) { return "$($entries.Count) entries start with $Name" }
    $entry = $entries[0]
    if ($entry.SystemComponent -eq 1) { return "$($entry.Key) is hidden from Settings" }
    $program = (Split-CommandLine $entry.Uninstall).Program
    if (-not (Get-Command -Name $program -CommandType Application -ErrorAction SilentlyContinue)) {
        return "it would run $program, which is not there"
    }
    return ''
}

function Shortcut-Problem([string]$Relative, [string]$Root, [string]$Program) {
    $path = Join-Path $script:StartMenu $Relative
    if (-not (Test-Path -LiteralPath $path)) { return "$path is not there" }
    $target = (New-Object -ComObject WScript.Shell).CreateShortcut($path).TargetPath
    if (-not (Is-Inside $target $Root)) { return "it points at $target" }
    if (-not (Test-Path -LiteralPath $target)) { return "it points at $target, which is not there" }
    # And what Windows does with it.
    Start-Process -FilePath $path -ErrorAction Stop
    if (-not (Started-From $Root $Program)) { return "starting it did not start $Program from the app's folder" }
    return ''
}

function Machine-Path-Problem([string]$Program, [string]$Root) {
    $machine = [Environment]::GetEnvironmentVariable('Path', 'Machine')
    $saved = $env:Path
    $env:Path = $machine
    try {
        $ErrorActionPreference = 'Continue'
        $found = @(& where.exe $Program 2>$null)
    } finally {
        $env:Path = $saved
    }
    if (@($found | Where-Object { Is-Inside $_ $Root }).Count -eq 0) { return "a new prompt would find: [$($found -join ', ')]" }
    return ''
}

# From a process of the bitness of the DLL, or Windows cannot load it. An Arm PC has no x64 PowerShell,
# so there a small x64 program asks, which Windows runs under its emulator.
function Module-Activated([string]$Clsid, [string]$Module) {
    if ($script:Architecture -eq 'ARM64') {
        if (-not $script:ComProbe) {
            $work = New-WorkFolder
            $source = Join-Path $work 'probe.cs'
            Set-Content -LiteralPath $source -Value @'
using System;
using System.Diagnostics;
class Probe {
    static int Main(string[] args) {
        Activator.CreateInstance(Type.GetTypeFromCLSID(new Guid(args[0])));
        foreach (ProcessModule module in Process.GetCurrentProcess().Modules)
            if (string.Equals(module.ModuleName, args[1], StringComparison.OrdinalIgnoreCase))
                Console.WriteLine(module.FileName);
        return 0;
    }
}
'@
            $probe = Join-Path $work 'probe.exe'
            $built = Run-Program "$env:SystemRoot\Microsoft.NET\Framework64\v4.0.30319\csc.exe" @('/nologo', '/platform:x64', "/out:$probe", $source)
            if ($built.Code -ne 0) { throw "could not build the x64 probe: $($built.Output)" }
            $script:ComProbe = $probe
        }
        return (Run-Program $script:ComProbe @($Clsid, $Module))
    }
    $script = "`$ErrorActionPreference = 'Stop'; [void][Activator]::CreateInstance([type]::GetTypeFromCLSID('$Clsid')); foreach (`$m in [Diagnostics.Process]::GetCurrentProcess().Modules) { if (`$m.ModuleName -ieq '$Module') { `$m.FileName } }"
    return (Run-Program "$env:SystemRoot\System32\WindowsPowerShell\v1.0\powershell.exe" @('-NoProfile', '-NonInteractive', '-Command', $script))
}

function Service-Problem([string]$Name, [string]$Root, [scriptblock]$Serve) {
    $service = Get-CimInstance Win32_Service -Filter "Name='$Name'"
    if (-not $service) { return "there is no service $Name" }
    if ($service.PathName.IndexOf($Root, [StringComparison]::OrdinalIgnoreCase) -lt 0) { return "it runs $($service.PathName)" }
    Start-Service -Name $Name -ErrorAction Stop
    try {
        if (-not (Wait-Until { (Get-Service -Name $Name).Status -eq 'Running' } 30)) {
            return "it is $((Get-Service -Name $Name).Status), not running"
        }
        Start-Sleep -Seconds 5
        $service = Get-CimInstance Win32_Service -Filter "Name='$Name'"
        if ($service.State -ne 'Running') { return "it stopped by itself: $($service.State), exit code $($service.ExitCode)" }
        $process = Get-CimInstance Win32_Process -Filter "ProcessId=$($service.ProcessId)"
        if (-not (Is-Inside $process.ExecutablePath $Root)) { return "its process is $($process.ExecutablePath)" }
        $problem = & $Serve
        if ($problem) { return "$problem" }
    } finally {
        Stop-Service -Name $Name -Force -ErrorAction SilentlyContinue
    }
    if (-not (Wait-Until { (Get-Service -Name $Name).Status -eq 'Stopped' } 30)) {
        return "it would not stop: $((Get-Service -Name $Name).Status)"
    }
    return ''
}

# A real MQTT 3.1.1 CONNECT, by hand, and the broker's answer to it.
function Mqtt-Problem([int]$Port) {
    $client = [Net.Sockets.TcpClient]::new()
    try {
        $client.Connect('127.0.0.1', $Port)
        $stream = $client.GetStream()
        $stream.ReadTimeout = 5000
        [byte[]]$connect = 0x10, 15, 0, 4, 0x4D, 0x51, 0x54, 0x54, 4, 2, 0, 60, 0, 3, 0x77, 0x61, 0x6D
        $stream.Write($connect, 0, $connect.Length)
        $answer = [byte[]]::new(4)
        $read = 0
        while ($read -lt 4) {
            $got = $stream.Read($answer, $read, 4 - $read)
            if ($got -le 0) { break }
            $read += $got
        }
        if ($read -ne 4 -or $answer[0] -ne 0x20 -or $answer[1] -ne 2 -or $answer[3] -ne 0) {
            return "the broker answered [$($answer -join ' ')]"
        }
        return ''
    } finally {
        $client.Dispose()
    }
}

# --- What is asked of each app.

$script:Checks = @{
    '7zip' = @(
        @{ Name = 'CLI: 7z packs and unpacks, byte for byte'; Test = {
                param($Root)
                $work = New-WorkFolder
                $in = Join-Path $work 'in'
                New-Item -ItemType Directory -Path $in | Out-Null
                $bytes = [byte[]]::new(2097152)
                [Random]::new(7).NextBytes($bytes)
                [IO.File]::WriteAllBytes((Join-Path $in 'noise.bin'), $bytes)
                [IO.File]::WriteAllText((Join-Path $in 'Größe.txt'), 'a name that is not ASCII')
                $packed = Run-Program "$Root\7z.exe" @('a', '-t7z', "$work\a.7z", "$in\*")
                if ($packed.Code -ne 0) { return "packing exited with $($packed.Code): $($packed.Output)" }
                $unpacked = Run-Program "$Root\7z.exe" @('x', "-o$work\out", "$work\a.7z")
                if ($unpacked.Code -ne 0) { return "unpacking exited with $($unpacked.Code): $($unpacked.Output)" }
                foreach ($name in 'noise.bin', 'Größe.txt') {
                    if ((Sha256-Of (Join-Path $in $name)) -ne (Sha256-Of (Join-Path "$work\out" $name))) { return "$name came back different" }
                }
                return ''
            }
        },
        @{ Name = 'COM: Windows activates the shell extension by CLSID, from the app''s folder'; Test = {
                param($Root)
                $run = Module-Activated $script:SevenZipShellExtension '7-zip.dll'
                if ($run.Code -ne 0) { return "activation exited with $($run.Code): $($run.Output)" }
                $loaded = $run.Output.Trim()
                if (-not (Is-Inside $loaded $Root)) { return "it loaded [$loaded]" }
                return ''
            }
        },
        @{ Name = 'Start Menu: the shortcut leads to the program, and starts it'; Test = {
                param($Root)
                return (Shortcut-Problem '7-Zip\7-Zip File Manager.lnk' $Root '7zFM.exe')
            }
        },
        @{ Name = 'App Paths: Windows finds the program by name alone'; Test = {
                param($Root)
                # A program can also be found on Path, which says nothing of this entry.
                $registered = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey('SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\7zFM.exe')
                if (-not $registered) { return 'there is no entry for it' }
                $program = [string]$registered.GetValue('')
                if (-not ((Is-Inside $program $Root) -and (Test-Path -LiteralPath $program))) { return "the entry says [$program]" }
                Start-Process -FilePath '7zFM.exe' -ErrorAction Stop
                if (-not (Started-From $Root '7zFM.exe')) { return 'it did not start from the app''s folder' }
                return ''
            }
        },
        @{ Name = 'Settings: the entry is listed and its uninstaller is there'; Test = {
                param($Root)
                return (Entry-Problem '7-Zip')
            }
        }
    )
    'putty' = @(
        @{ Name = 'CLI: plink opens a real connection and says its version'; Test = {
                param($Root)
                $listener = [Net.Sockets.TcpListener]::new([Net.IPAddress]::Loopback, 0)
                $listener.Start()
                try {
                    $client = Start-Hidden "$Root\plink.exe" "-batch -ssh -P $($listener.LocalEndpoint.Port) 127.0.0.1"
                    try {
                        if (-not (Wait-Until { $listener.Pending() } 15)) { return 'it never connected' }
                        $socket = $listener.AcceptTcpClient()
                        $stream = $socket.GetStream()
                        $stream.ReadTimeout = 10000
                        $banner = [Text.Encoding]::ASCII.GetBytes("SSH-2.0-WamCheck`r`n")
                        $stream.Write($banner, 0, $banner.Length)
                        $buffer = [byte[]]::new(256)
                        $got = $stream.Read($buffer, 0, 256)
                        $said = [Text.Encoding]::ASCII.GetString($buffer, 0, $got)
                        $socket.Dispose()
                        if ($said -notmatch '^SSH-2\.0-PuTTY_Release_0\.85') { return "it said: $said" }
                    } finally {
                        Stop-Process -Id $client.Id -Force -ErrorAction SilentlyContinue
                    }
                } finally {
                    $listener.Stop()
                }
                return ''
            }
        },
        @{ Name = 'File type: Windows says .ppk opens with the app''s programs'; Test = {
                param($Root)
                $open = [WamAssociation]::ProgramFor('.ppk', 'open')
                if (-not ((Is-Inside $open $Root) -and (Test-Path -LiteralPath $open))) { return "open goes to [$open]" }
                $edit = [WamAssociation]::ProgramFor('.ppk', 'edit')
                if (-not ((Is-Inside $edit $Root) -and (Test-Path -LiteralPath $edit))) { return "edit goes to [$edit]" }
                return ''
            }
        },
        @{ Name = 'File type: opening a .ppk starts Pageant with it'; Test = {
                param($Root)
                # With nothing to open it, Windows would ask a person, and this would wait for ever.
                $open = [WamAssociation]::ProgramFor('.ppk', 'open')
                if (-not (Is-Inside $open $Root)) { return "nothing of the app's would open it: [$open]" }
                $work = New-WorkFolder
                $file = Join-Path $work 'wamcheck.ppk'
                Set-Content -LiteralPath $file -Value 'not a real key'
                Start-Process -FilePath $file -ErrorAction Stop
                if (-not (Started-From $Root 'pageant.exe' 'wamcheck')) { return 'Pageant did not start from the app''s folder with that file' }
                return ''
            }
        },
        @{ Name = 'Start Menu: the shortcut leads to the program, and starts it'; Test = {
                param($Root)
                return (Shortcut-Problem 'PuTTY (64-bit)\PuTTY.lnk' $Root 'putty.exe')
            }
        },
        @{ Name = 'PATH: a new prompt finds plink in the app''s folder'; Test = {
                param($Root)
                return (Machine-Path-Problem 'plink.exe' $Root)
            }
        },
        @{ Name = 'Settings: the entry is listed and its uninstaller is there'; Test = {
                param($Root)
                return (Entry-Problem 'PuTTY')
            }
        }
    )
    'mosquitto' = @(
        @{ Name = 'CLI: the broker passes a message from its publisher to its subscriber'; Test = {
                param($Root)
                $port = Free-Port
                $broker = Start-Hidden "$Root\mosquitto.exe" "-c `"$Root\mosquitto.conf`" -p $port"
                try {
                    $listening = Wait-Until {
                        $probe = [Net.Sockets.TcpClient]::new()
                        try { $probe.Connect('127.0.0.1', $port); $true } catch { $false } finally { $probe.Dispose() }
                    } 15
                    if (-not $listening) { return "it never listened: exited=$($broker.HasExited) code=$(if ($broker.HasExited) { $broker.ExitCode })" }
                    $problem = Mqtt-Problem $port
                    if ($problem) { return $problem }
                    $subscriber = Start-Hidden "$Root\mosquitto_sub.exe" "-h 127.0.0.1 -p $port -t wam/check -C 1" -CaptureOutput
                    try {
                        $read = $subscriber.StandardOutput.ReadToEndAsync()
                        $delivered = Wait-Until {
                            $publisher = Start-Hidden "$Root\mosquitto_pub.exe" "-h 127.0.0.1 -p $port -t wam/check -m from-the-publisher"
                            $publisher.WaitForExit(5000) | Out-Null
                            $subscriber.HasExited
                        } 20
                        if (-not $delivered) { return 'the subscriber never got it' }
                        $said = $read.GetAwaiter().GetResult().Trim()
                        if ($said -ne 'from-the-publisher') { return "the subscriber got [$said]" }
                    } finally {
                        if (-not $subscriber.HasExited) { $subscriber.Kill() }
                    }
                } finally {
                    if (-not $broker.HasExited) { $broker.Kill() }
                }
                return ''
            }
        },
        @{ Name = 'Environment: MOSQUITTO_DIR is the app''s folder'; Test = {
                param($Root)
                $value = [Environment]::GetEnvironmentVariable('MOSQUITTO_DIR', 'Machine')
                if ($value -ine $Root.TrimEnd('\')) { return "it is [$value]" }
                return ''
            }
        },
        @{ Name = 'Service: it starts, stays up, answers a real MQTT client, and stops'; Test = {
                param($Root)
                return (Service-Problem 'mosquitto' $Root { Mqtt-Problem 1883 })
            }
        },
        @{ Name = 'Settings: the entry is listed and its uninstaller is there'; Test = {
                param($Root)
                return (Entry-Problem 'Eclipse Mosquitto')
            }
        }
    )
    'windows_exporter' = @(
        @{ Name = 'Folder: the empty one its setup made is there, with nothing in it'; Test = {
                param($Root)
                $folder = Join-Path $Root 'textfile_inputs'
                if (-not (Test-Path -LiteralPath $folder -PathType Container)) { return "$folder is not a folder here" }
                $inside = @(Get-ChildItem -LiteralPath $folder -Force)
                if ($inside.Count -gt 0) { return "it holds $($inside.Name -join ', ')" }
                return ''
            }
        },
        @{ Name = 'Firewall: Windows has the rule, for the program in the app''s folder and its port'; Test = {
                param($Root)
                $rules = @(Get-NetFirewallRule -DisplayName 'windows_exporter' -ErrorAction SilentlyContinue)
                if ($rules.Count -ne 1) { return "$($rules.Count) rules have that name" }
                if ($rules[0].Enabled -ne 'True' -or $rules[0].Direction -ne 'Inbound' -or $rules[0].Action -ne 'Allow') {
                    return "it is $($rules[0].Enabled), $($rules[0].Direction), $($rules[0].Action)"
                }
                $program = ($rules[0] | Get-NetFirewallApplicationFilter).Program
                if (-not (Is-Inside $program $Root)) { return "it is for [$program]" }
                $port = ($rules[0] | Get-NetFirewallPortFilter).LocalPort
                if ($port -ne '9182') { return "it is for port [$port]" }
                return ''
            }
        },
        @{ Name = 'Service: it starts, stays up, answers a real HTTP client, and stops'; Test = {
                param($Root)
                return (Service-Problem 'windows_exporter' $Root {
                        $http = [Net.Http.HttpClient]::new()
                        $http.Timeout = [TimeSpan]::FromSeconds(10)
                        $answer = @{ Page = '' }
                        $answered = Wait-Until {
                            try { $answer.Page = $http.GetStringAsync('http://127.0.0.1:9182/metrics').GetAwaiter().GetResult(); $true } catch { $false }
                        } 45
                        $http.Dispose()
                        if (-not $answered) { return 'it never answered on port 9182' }
                        if ($answer.Page -notmatch 'windows_exporter_build_info\{.*version="0\.32\.1"') { return "its answer was not its metrics: $($answer.Page.Length) characters" }
                        $collectors = @($answer.Page -split "`n" | Where-Object { $_ -match '^windows_exporter_collector_success' })
                        Write-Host "      what it says of its own collectors: $($collectors -join '; ')"
                    })
            }
        },
        @{ Name = 'Settings: the entry is listed and its uninstaller is there'; Test = {
                param($Root)
                return (Entry-Problem 'windows_exporter')
            }
        }
    )
    'git-lfs' = @(
        @{ Name = 'CLI: git-lfs works out a file''s pointer, by the same hash as .NET'; Test = {
                param($Root)
                $work = New-WorkFolder
                $file = Join-Path $work 'sample.bin'
                $bytes = [byte[]]::new(1048576)
                [Random]::new(11).NextBytes($bytes)
                [IO.File]::WriteAllBytes($file, $bytes)
                $run = Run-Program "$Root\git-lfs.exe" @('pointer', "--file=$file")
                if ($run.Code -ne 0) { return "it exited with $($run.Code): $($run.Output)" }
                if ($run.Output -notmatch "oid sha256:$(Sha256-Of $file)") { return "it said: $($run.Output)" }
                if ($run.Output -notmatch 'size 1048576') { return "it said: $($run.Output)" }
                $version = Run-Program "$Root\git-lfs.exe" @('version')
                if ($version.Output -notmatch 'git-lfs/3\.8\.1') { return "its version says: $($version.Output)" }
                return ''
            }
        },
        @{ Name = 'PATH: a new prompt finds git-lfs in the app''s folder'; Test = {
                param($Root)
                return (Machine-Path-Problem 'git-lfs.exe' $Root)
            }
        },
        @{ Name = 'Settings: the entry is listed and its uninstaller is there'; Test = {
                param($Root)
                return (Entry-Problem 'Git LFS')
            }
        }
    )
}

function Check-Result($Check, [string]$Root) {
    try {
        $said = @(& $Check.Test $Root)
        if ($said.Count -eq 0) { return '' }
        return [string]$said[-1]
    } catch {
        return "threw: $($_.Exception.Message)"
    }
}

# --- What is on the machine, to see that an uninstall left nothing.

function Key-Names([string]$Path, $Hive = [Microsoft.Win32.Registry]::LocalMachine) {
    $key = $Hive.OpenSubKey($Path)
    if (-not $key) { return @() }
    return @($key.GetSubKeyNames() | ForEach-Object { "$($Hive.Name)\$Path\$_" })
}

function Value-Names([string]$Path) {
    $key = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey($Path)
    if (-not $key) { return @() }
    return @($key.GetValueNames() | ForEach-Object { "$Path : $_" })
}

# Each value a package says it sets, as it is on this PC now, so that what an uninstall leaves is seen
# in what each one holds, wherever under a key that everyone shares it is.
function Captured-Values([string]$Manifests, $Apps) {
    foreach ($app in $Apps) {
        $manifest = Get-Content -Raw -LiteralPath (Join-Path $Manifests "$($app.id).json") | ConvertFrom-Json
        foreach ($entry in $manifest.registry_keys_exported) {
            $hive = if ($entry.hive -match 'current') { [Microsoft.Win32.Registry]::CurrentUser } else { [Microsoft.Win32.Registry]::LocalMachine }
            $key = $hive.OpenSubKey($entry.key_path)
            if (-not $key) { continue }
            if ($null -eq $entry.value_name) {
                "$($hive.Name)\$($entry.key_path)"
                continue
            }
            $name = if ($entry.value_name -eq '(Default)') { '' } else { $entry.value_name }
            $value = $key.GetValue($name, $null, 'DoNotExpandEnvironmentNames')
            if ($null -ne $value) {
                # By its kind and in a form where no two different values read the same.
                "$($hive.Name)\$($entry.key_path) : $($entry.value_name) = $($key.GetValueKind($name)) $(ConvertTo-Json -InputObject $value -Compress)"
            }
        }
    }
}

# What someone made of 7-Zip on the PC the installers were built on. See install-real-apps.ps1.
$script:PersonalTaskPath = '\WAM Personal\'
$script:PersonalTaskName = 'WAM Personal Task'
$script:PlainShortcut = 'WAM Plain 7-Zip.lnk'
$script:PersonalShortcut = 'WAM Personal 7-Zip.lnk'

# Classes that hold nothing but an AppID, as Windows and what it runs in the background add and take
# away by themselves while a job runs. Seen at each look at the machine, as one that has gone cannot be
# asked what it held.
$script:ClassesWithOnlyAnAppId = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)

function Note-Classes-With-Only-An-AppId([string]$Path) {
    $classes = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey($Path)
    if (-not $classes) { return }
    foreach ($name in $classes.GetSubKeyNames()) {
        try { $class = $classes.OpenSubKey($name) } catch { continue }
        if (-not $class) { continue }
        $values = @($class.GetValueNames())
        if ($class.SubKeyCount -eq 0 -and $values.Count -eq 1 -and $values[0] -eq 'AppID') {
            [void]$script:ClassesWithOnlyAnAppId.Add("HKEY_LOCAL_MACHINE\$Path\$name")
        }
        $class.Dispose()
    }
    $classes.Dispose()
}

# Whether a class that is new or gone is none of these packages' doing: it is named in none of what they
# hold, and holds nothing but an AppID. Anything else is what an uninstall left, or took.
function Is-Windows-Own-Class([string]$Kind, [string]$Item) {
    if ($Kind -ne 'Classes' -or $Item -notmatch '\\CLSID\\(\{[0-9A-Fa-f-]+\})$') { return $false }
    $clsid = $Matches[1]
    if ($script:WhatThePackagesHold -and $script:WhatThePackagesHold.IndexOf($clsid, [StringComparison]::OrdinalIgnoreCase) -ge 0) {
        return $false
    }
    return $script:ClassesWithOnlyAnAppId.Contains($Item)
}

function Get-MachineState([string]$Manifests, $Apps) {
    $script:WhatThePackagesHold = (@($Apps) | ForEach-Object {
            Get-Content -Raw -LiteralPath (Join-Path $Manifests "$($_.id).json")
        }) -join "`n"
    Note-Classes-With-Only-An-AppId 'SOFTWARE\Classes\CLSID'
    Note-Classes-With-Only-An-AppId 'SOFTWARE\WOW6432Node\Classes\CLSID'
    $environmentKey = 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment'
    $environment = [Microsoft.Win32.Registry]::LocalMachine.OpenSubKey($environmentKey)
    $mine = [Microsoft.Win32.Registry]::CurrentUser
    $myEnvironment = $mine.OpenSubKey('Environment')
    $names = @{
        'What they set'   = @(Captured-Values $Manifests $Apps)
        'This account''s' = @(
            Key-Names 'Software\Microsoft\Windows\CurrentVersion\Uninstall' $mine
            $myEnvironment.GetValueNames() | ForEach-Object { "Environment $_=$($myEnvironment.GetValue($_, '', 'DoNotExpandEnvironmentNames'))" }
            Get-ChildItem -LiteralPath ([Environment]::GetFolderPath('Programs')) -Recurse -Force | ForEach-Object FullName
        )
        'Folders'         = @(
            Get-ChildItem -LiteralPath 'C:\Program Files', 'C:\Program Files (x86)' -Force -ErrorAction SilentlyContinue | ForEach-Object FullName
            Get-ChildItem -LiteralPath 'C:\ProgramData' -Force | Where-Object Name -ne 'WinAppMigrator' | ForEach-Object FullName
        )
        'Start Menu'      = @(Get-ChildItem -LiteralPath $script:StartMenu -Recurse -Force | ForEach-Object FullName)
        'Services'        = @(Key-Names 'SYSTEM\CurrentControlSet\Services')
        'Uninstall'       = @(
            Key-Names 'SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall'
            Key-Names 'SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall'
        )
        'Software'        = @(Key-Names 'SOFTWARE'; Key-Names 'SOFTWARE\WOW6432Node')
        'Classes'         = @(Key-Names 'SOFTWARE\Classes'; Key-Names 'SOFTWARE\Classes\CLSID'; Key-Names 'SOFTWARE\WOW6432Node\Classes\CLSID')
        'App Paths'       = @(Key-Names 'SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths')
        'Shell approved'  = @(Value-Names 'SOFTWARE\Microsoft\Windows\CurrentVersion\Shell Extensions\Approved')
        'Firewall'        = @(Value-Names 'SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\FirewallRules')
        'Tasks'           = @(Get-ScheduledTask | ForEach-Object { "$($_.TaskPath)$($_.TaskName)" })
        'Environment'     = @($environment.GetValueNames() | ForEach-Object { "$_=$($environment.GetValue($_, '', 'DoNotExpandEnvironmentNames'))" })
    }
    $state = @{}
    foreach ($kind in $names.Keys) { $state[$kind] = @($names[$kind] | Sort-Object -Unique -CaseSensitive) }
    return $state
}

# So that one nobody can account for can be told from the log: what it holds, and who it says it is.
function What-Key-Holds([string]$Kind, [string]$Item) {
    if ($Kind -notin 'Classes', 'Software', 'App Paths', 'Services', 'Uninstall' -or $Item -notmatch '^HKEY_') { return '' }
    $path = "Registry::$Item"
    if (-not (Test-Path -LiteralPath $path)) { return ' (it is gone by now)' }
    $parts = [Collections.Generic.List[string]]::new()
    try {
        $key = Get-Item -LiteralPath $path
        foreach ($name in $key.GetValueNames()) { $parts.Add("$(if ($name) { $name } else { '(default)' })=$($key.GetValue($name))") }
        foreach ($child in 'InprocServer32', 'LocalServer32', 'InProcServer32') {
            $server = Join-Path $path $child
            if (Test-Path -LiteralPath $server) { $parts.Add("$child=$((Get-Item -LiteralPath $server).GetValue(''))") }
        }
    } catch { $parts.Add("could not read: $($_.Exception.Message)") }
    return " (holds: $($parts -join '; '))"
}

function State-Differences($Before, $After) {
    foreach ($kind in ($Before.Keys | Sort-Object)) {
        # What a value holds has to come back exactly. What a thing is called goes by no case, as Windows does.
        $comparer = if ($kind -in 'What they set', 'Environment') { [StringComparer]::Ordinal } else { [StringComparer]::OrdinalIgnoreCase }
        $set = [Collections.Generic.HashSet[string]]::new([string[]]$Before[$kind], $comparer)
        $now = [Collections.Generic.HashSet[string]]::new([string[]]$After[$kind], $comparer)
        foreach ($item in $After[$kind]) {
            if ($set.Contains($item)) { continue }
            # Windows adds these itself for its own packages, whenever it first starts one.
            if ($kind -eq 'Firewall' -and $item -match '_8wekyb3d8bbwe') {
                Write-Host "  (Windows' own, and not counted: $item)"
            } elseif (Is-Windows-Own-Class $kind $item) {
                Write-Host "  (Windows' own, and not counted: new, with only an AppID: $item$(What-Key-Holds $kind $item))"
            } else {
                "$kind, new: $item$(What-Key-Holds $kind $item)"
            }
        }
        foreach ($item in $Before[$kind]) {
            if ($now.Contains($item)) { continue }
            if (Is-Windows-Own-Class $kind $item) {
                Write-Host "  (Windows' own, and not counted: gone, with only an AppID: $item)"
            } else {
                "$kind, gone: $item"
            }
        }
    }
}
