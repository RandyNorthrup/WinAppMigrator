# Opens the real programs, with real windows, on a runner with no graphics card. Everything else tests
# the screens with no window at all, which says nothing of whether one opens. It clicks nothing and types
# nothing: it waits for the window, takes a picture of what is on the screen, and asks it to close. An
# installer given no arguments waits at its first screen, so nothing is installed.
#
# Each has to fit in the part of the screen a window can use, so that what has to be clicked can be, and be
# in the mode Windows is in, which this sets both ways. It then opens each with graphics that cannot start,
# to see what a person is told, and with the environment naming a folder of Direct3D's code to run instead,
# which must not be run.
#
# Run only by the job that downloads these, on a runner that is thrown away afterward.
param(
    [Parameter(Mandatory)][string]$App,
    [Parameter(Mandatory)][string]$Installers,
    [Parameter(Mandatory)][string]$Pictures,
    [Parameter(Mandatory)][string]$Planted
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'real-app-checks.ps1')
Require-ThrowawayRunner

$script:failures = [Collections.Generic.List[string]]::new()
New-Item -ItemType Directory -Force -Path $Pictures | Out-Null

function Check([string]$Label, [bool]$Passed, [string]$Detail = '') {
    if ($Passed) {
        Write-Host "  ok: $Label"
    } else {
        Write-Host "  FAILED: $Label`: $Detail"
        $script:failures.Add("$Label`: $Detail")
    }
}

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
[StructLayout(LayoutKind.Sequential)]
public struct WamRect { public int Left, Top, Right, Bottom; }
public static class WamWindow {
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr window, out WamRect rect);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr window);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr window);
    [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr window, IntPtr device, uint flags);
    [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
    // 9 is DWMWA_EXTENDED_FRAME_BOUNDS: what can be seen of a window, without the borders it only has for
    // being taken hold of, which GetWindowRect counts and which hang over the edge of a maximized one.
    [DllImport("dwmapi.dll")] static extern int DwmGetWindowAttribute(IntPtr window, int attribute, out WamRect rect, int size);
    public static bool SeenAt(IntPtr window, out WamRect rect) {
        return DwmGetWindowAttribute(window, 9, out rect, Marshal.SizeOf(typeof(WamRect))) == 0;
    }

    delegate bool EachWindow(IntPtr window, IntPtr unused);
    [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr window, EachWindow each, IntPtr unused);
    [DllImport("user32.dll")] static extern bool EnumWindows(EachWindow each, IntPtr unused);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr window, out uint process);
    [DllImport("user32.dll")] public static extern bool PostMessageW(IntPtr window, uint message, IntPtr first, IntPtr second);

    // What a person would call a program's window: the biggest it has showing. What .NET calls its main
    // window can be a little one that something it uses made, on Windows Server 2022 at least.
    public static IntPtr BiggestOf(int processId) {
        IntPtr biggest = IntPtr.Zero;
        long most = 0;
        EnumWindows((window, unused) => {
            uint owner;
            GetWindowThreadProcessId(window, out owner);
            WamRect rect;
            if (owner == processId && IsWindowVisible(window) && GetWindowRect(window, out rect)) {
                long area = (long)(rect.Right - rect.Left) * (rect.Bottom - rect.Top);
                if (area > most) { most = area; biggest = window; }
            }
            return true;
        }, IntPtr.Zero);
        return biggest;
    }
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern IntPtr SendMessageTimeoutW(IntPtr window, uint message, IntPtr most, System.Text.StringBuilder text, uint flags, uint milliseconds, out IntPtr result);

    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowTextW(IntPtr window, System.Text.StringBuilder text, int most);
    public static string TitleOf(IntPtr window) {
        var text = new System.Text.StringBuilder(512);
        GetWindowTextW(window, text, text.Capacity);
        return text.ToString();
    }

    // What the windows in this one say, which is what a plain message box's text is. They are another
    // program's, so this asks each with WM_GETTEXT, as GetWindowText would not answer for them.
    public static string TextIn(IntPtr window) {
        var all = new System.Text.StringBuilder();
        EnumChildWindows(window, (child, unused) => {
            var text = new System.Text.StringBuilder(4096);
            IntPtr got;
            SendMessageTimeoutW(child, 0x000D, (IntPtr)text.Capacity, text, 2, 5000, out got);
            all.AppendLine(text.ToString());
            return true;
        }, IntPtr.Zero);
        return all.ToString();
    }
}
'@

# 2 is PW_RENDERFULLCONTENT, without which a window drawn by the graphics card comes out blank.
$PrintFullContent = 2

# What the window draws, asked of the window itself.
function Picture-Asked([IntPtr]$Window, $Rect) {
    $picture = [Drawing.Bitmap]::new($Rect.Right - $Rect.Left, $Rect.Bottom - $Rect.Top)
    $graphics = [Drawing.Graphics]::FromImage($picture)
    $device = $graphics.GetHdc()
    $drawn = [WamWindow]::PrintWindow($Window, $device, $PrintFullContent)
    $graphics.ReleaseHdc($device)
    $graphics.Dispose()
    if (-not $drawn) { throw 'the window would not draw itself' }
    return $picture
}

# What is on the screen where it is.
function Picture-On-Screen($Rect) {
    $picture = [Drawing.Bitmap]::new($Rect.Right - $Rect.Left, $Rect.Bottom - $Rect.Top)
    $graphics = [Drawing.Graphics]::FromImage($picture)
    $graphics.CopyFromScreen($Rect.Left, $Rect.Top, 0, 0, $picture.Size)
    $graphics.Dispose()
    return $picture
}

# How many colors a picture has, and how much of it is the commonest, going by a grid of points.
function Colors-Of($Picture) {
    $counts = @{}
    $total = 0
    for ($y = 0; $y -lt $Picture.Height; $y += 4) {
        for ($x = 0; $x -lt $Picture.Width; $x += 4) {
            $counts[$Picture.GetPixel($x, $y).ToArgb()]++
            $total++
        }
    }
    $most = ($counts.Values | Measure-Object -Maximum).Maximum
    return @($counts.Count, $(if ($total) { [int]($most * 100 / $total) } else { 100 }))
}

# How bright what is below the title bar is, from 0 to 255, going by a grid of points. The title bar is Windows'.
function Brightness-Of($Picture) {
    $sum = 0.0
    $count = 0
    for ($y = 48; $y -lt $Picture.Height; $y += 8) {
        for ($x = 8; $x -lt $Picture.Width; $x += 8) {
            $sum += $Picture.GetPixel($x, $y).GetBrightness() * 255
            $count++
        }
    }
    return $(if ($count) { [int]($sum / $count) } else { 0 })
}

# Whether a picture has anything drawn in it. A blank window is one color, or nearly.
function Is-Drawn($Colors, [int]$AtLeast) { return $Colors[0] -ge $AtLeast -and $Colors[1] -le 98 }

# Open a program, wait for its window, take a picture, ask it to close, and say what happened.
function Open-And-Look([string]$Program, [string]$Name, [string]$WindowTitle, [int]$MinimumColors, [int]$Settle, [string[]]$Arguments = @(), [switch]$HasToFit) {
    Write-Host "`n--- $Name`: $Program $Arguments"
    $starting = @{ FilePath = $Program; PassThru = $true }
    if ($Arguments.Count -gt 0) { $starting.ArgumentList = $Arguments }
    $process = Start-Process @starting
    $state = @{ Window = [IntPtr]::Zero }
    $opened = Wait-Until {
        $process.Refresh()
        if ($process.HasExited) { return $true }
        $window = [WamWindow]::BiggestOf($process.Id)
        if ($window -ne [IntPtr]::Zero) {
            $state.Window = $window
            return $true
        }
        return $false
    } 120
    if ($process.HasExited -or -not $opened -or $state.Window -eq [IntPtr]::Zero) {
        $why = if ($process.HasExited) { "it exited with $($process.ExitCode)" } else { 'no window came within two minutes' }
        Check "$Name`: a window opened" $false $why
        if (-not $process.HasExited) { $process.Kill() }
        return $null
    }
    Check "$Name`: a window opened" $true
    # It has to get as far as drawing, and what it shows first it has to go and find. What first shows
    # can be a little window of something it uses, and on a slow PC it can be a while before there is more.
    Start-Sleep -Seconds $Settle
    Wait-Until {
        $process.Refresh()
        $process.HasExited -or [WamWindow]::TitleOf([WamWindow]::BiggestOf($process.Id)) -eq $WindowTitle
    } 60 | Out-Null
    $process.Refresh()
    Check "$Name`: it is still running" (-not $process.HasExited) "exit $(if ($process.HasExited) { $process.ExitCode })"
    if ($process.HasExited) { return $null }
    $state.Window = [WamWindow]::BiggestOf($process.Id)
    $title = [WamWindow]::TitleOf($state.Window)
    Write-Host "  its window says it is [$title]"
    Check "$Name`: its window is called what it should be" ($title -eq $WindowTitle) "[$title]"

    [WamWindow]::SetForegroundWindow($state.Window) | Out-Null
    Start-Sleep -Seconds 1
    $rect = [WamRect]::new()
    Check "$Name`: it has a place on the screen" ([WamWindow]::GetWindowRect($state.Window, [ref]$rect)) ''
    $width = $rect.Right - $rect.Left
    $height = $rect.Bottom - $rect.Top
    # What is on the screen where it is, is only of it if nothing is in front of it.
    $inFront = [WamWindow]::GetForegroundWindow() -eq $state.Window
    Write-Host "  it is $width by $height, at $($rect.Left), $($rect.Top), and $(if ($inFront) { 'in front' } else { 'not in front' })"
    Check "$Name`: it is big enough to be a window" ($width -ge 300 -and $height -ge 150) "$width by $height"
    if ($HasToFit) {
        $seenAt = [WamRect]::new()
        $area = [Windows.Forms.Screen]::FromHandle($state.Window).WorkingArea
        $known = [WamWindow]::SeenAt($state.Window, [ref]$seenAt)
        $where = "from $($seenAt.Left), $($seenAt.Top) to $($seenAt.Right), $($seenAt.Bottom), in what a window can use, from $($area.Left), $($area.Top) to $($area.Right), $($area.Bottom)"
        Write-Host "  what can be seen of it is $where"
        Check "$Name`: all of it is on the screen, clear of the taskbar" ($known -and $seenAt.Left -ge $area.Left -and $seenAt.Top -ge $area.Top -and $seenAt.Right -le $area.Right -and $seenAt.Bottom -le $area.Bottom) $where
    }

    $seen = @{}
    foreach ($way in 'on the screen', 'asked of the window') {
        try {
            $picture = if ($way -eq 'on the screen') { Picture-On-Screen $rect } else { Picture-Asked $state.Window $rect }
            $colors = Colors-Of $picture
            Write-Host "  $way`: $($colors[0]) colors, the commonest is $($colors[1])% of it"
            if ($way -eq 'asked of the window') { $script:LastBrightness = Brightness-Of $picture }
            $file = Join-Path $Pictures "$Name, $way.png"
            $picture.Save($file, [Drawing.Imaging.ImageFormat]::Png)
            $picture.Dispose()
            $seen[$way] = (Is-Drawn $colors $MinimumColors) -and ($way -ne 'on the screen' -or $inFront)
        } catch {
            Write-Host "  $way`: could not be taken: $($_.Exception.Message)"
            $seen[$way] = $false
        }
    }
    Check "$Name`: what it drew is not blank" ($seen['on the screen'] -or $seen['asked of the window']) ''
    $script:LastWindow = $state.Window
    return $process
}

# 16 is WM_CLOSE, which is what clicking its close button sends.

function Ask-To-Close($Process, [string]$Name, [int]$ExpectedExit) {
    $Process.Refresh()
    if ($Process.HasExited) { return }
    $asked = [WamWindow]::PostMessageW($script:LastWindow, 16, [IntPtr]::Zero, [IntPtr]::Zero)
    $closed = $Process.WaitForExit(30000)
    Check "$Name`: it closes when asked" ($asked -and $closed) "asked $asked, closed $closed"
    if ($closed) {
        Check "$Name`: and exits with $ExpectedExit" ($Process.ExitCode -eq $ExpectedExit) "$($Process.ExitCode)"
    } else {
        $Process.Kill()
    }
}

function Latest-Log([string]$Starting) {
    return Get-ChildItem -LiteralPath 'C:\ProgramData\WinAppMigrator\logs' -Filter "$Starting*" -ErrorAction SilentlyContinue | Sort-Object LastWriteTime | Select-Object -Last 1
}

$versionKey = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
Write-Host "This PC: $($versionKey.ProductName), build $($versionKey.CurrentBuildNumber).$($versionKey.UBR), $script:Architecture"
$session = (Get-Process -Id $PID).SessionId
Write-Host "This runs in session $session, as $([Environment]::UserName), interactive: $([Environment]::UserInteractive)"
foreach ($display in @(Get-CimInstance Win32_VideoController)) {
    Write-Host "A display adapter here: $($display.Name), driver $($display.DriverVersion), $($display.CurrentHorizontalResolution) by $($display.CurrentVerticalResolution)"
}

$expected = Get-Content -Raw -LiteralPath (Join-Path $Installers 'expected.json') | ConvertFrom-Json
$installer = Join-Path $Installers $expected.app.installer

# Windows' own setting for what programs look like, which this runner is thrown away with.
$Personalize = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize'
function Set-WindowsMode([string]$Mode) {
    New-Item -Path $Personalize -Force -ErrorAction SilentlyContinue | Out-Null
    Set-ItemProperty -Path $Personalize -Name 'AppsUseLightTheme' -Value ([int]($Mode -eq 'light')) -Type DWord
}

# What is drawn is mostly its background, which is near white in light mode and near black in dark.
function Check-Mode([string]$Name, [string]$Mode) {
    $brightness = $script:LastBrightness
    Write-Host "  below its title bar it is $brightness bright"
    $is = $(if ($brightness -ge 128) { 'light' } else { 'dark' })
    Check "$Name`: it is in $Mode mode" ($is -eq $Mode) "it is $brightness bright"
}

Write-Host "`n=== The mode Windows is in, as this runner has it: $((Get-ItemProperty -Path $Personalize -ErrorAction SilentlyContinue).AppsUseLightTheme)"
Set-WindowsMode 'light'

Write-Host "`n=== The app, as it is released"
$opened = Open-And-Look $App 'The app' 'WinAppMigrator' 32 20 -HasToFit
if ($opened) {
    Check-Mode 'The app' 'light'
    # It starts its log in the background, which under emulation can take longer than all of this.
    [void](Wait-Until { Latest-Log 'app' } 120)
    Ask-To-Close $opened 'The app' 0
    $log = Latest-Log 'app'
    if ($log) {
        $said = @(Get-Content -LiteralPath $log.FullName | Where-Object { $_ -match 'graphics adapter' })
        $said | ForEach-Object { Write-Host "  its log: $_" }
        Check 'The app: it says which graphics adapter it picked' ($said.Count -gt 0) $log.FullName
    } else {
        Check 'The app: it wrote a log' $false 'none is there'
    }
}

Write-Host "`n=== An installer of the test app, waiting at its first screen"
$before = Test-Path -LiteralPath "C:\Program Files\$($expected.app.name)"
# Called what it is installing, once it has read that.
$opened = Open-And-Look $installer 'The installer' "Install $($expected.app.name)" 32 15 -HasToFit
if ($opened) {
    Check-Mode 'The installer' 'light'
    Ask-To-Close $opened 'The installer' 0
}
Check 'The installer: it installed nothing' ((Test-Path -LiteralPath "C:\Program Files\$($expected.app.name)") -eq $before) ''

Write-Host "`n=== An uninstaller, of an install there is no record of, which only says so"
$opened = Open-And-Look $installer 'The uninstaller' 'Uninstall' 8 8 -Arguments @('--uninstall', 'no-such-install') -HasToFit
if ($opened) {
    Check-Mode 'The uninstaller' 'light'
    Ask-To-Close $opened 'The uninstaller' 0
}

Write-Host "`n=== With Windows in dark mode"
Set-WindowsMode 'dark'
foreach ($case in @(
        @{ Program = $App; Name = 'The app, Windows dark'; Title = 'WinAppMigrator'; Settle = 20; Colors = 32; Arguments = @() },
        @{ Program = $installer; Name = 'The installer, Windows dark'; Title = "Install $($expected.app.name)"; Settle = 15; Colors = 32; Arguments = @() },
        @{ Program = $installer; Name = 'The uninstaller, Windows dark'; Title = 'Uninstall'; Settle = 8; Colors = 8; Arguments = @('--uninstall', 'no-such-install') }
    )) {
    $opened = Open-And-Look $case.Program $case.Name $case.Title $case.Colors $case.Settle -Arguments $case.Arguments -HasToFit
    if ($opened) {
        Check-Mode $case.Name 'dark'
        Ask-To-Close $opened $case.Name 0
    }
}

Write-Host "`n=== What a person chose in the app goes ahead of what Windows is in, in all three"
$chosen = Join-Path $env:LOCALAPPDATA 'WinAppMigrator\theme.txt'
New-Item -ItemType Directory -Force -Path (Split-Path $chosen) | Out-Null
Set-Content -LiteralPath $chosen -Value 'light' -NoNewline
foreach ($case in @(
        @{ Program = $App; Name = 'The app, chosen light with Windows dark'; Title = 'WinAppMigrator'; Settle = 20; Colors = 32; Arguments = @() },
        @{ Program = $installer; Name = 'The installer, chosen light with Windows dark'; Title = "Install $($expected.app.name)"; Settle = 15; Colors = 32; Arguments = @() },
        @{ Program = $installer; Name = 'The uninstaller, chosen light with Windows dark'; Title = 'Uninstall'; Settle = 8; Colors = 8; Arguments = @('--uninstall', 'no-such-install') }
    )) {
    $opened = Open-And-Look $case.Program $case.Name $case.Title $case.Colors $case.Settle -Arguments $case.Arguments -HasToFit
    if ($opened) {
        Check-Mode $case.Name 'light'
        Ask-To-Close $opened $case.Name 0
    }
}
Remove-Item -LiteralPath $chosen
Set-WindowsMode 'light'

Write-Host "`n=== With graphics that cannot start, what a person is told"
# wgpu goes by this, and these programs have no renderer but Direct3D 12.
$env:WGPU_BACKEND = 'vulkan'
foreach ($case in @(
        @{ Program = $App; Name = 'The app, with no graphics'; Title = 'WinAppMigrator'; Says = @('Direct3D 12') },
        @{ Program = $installer; Name = 'The installer, with no graphics'; Title = 'WinAppMigrator Installer'; Says = @('Direct3D 12', '--apply', '--target-root'); Arguments = @() },
        @{ Program = $installer; Name = 'The uninstaller, with no graphics'; Title = 'WinAppMigrator Uninstaller'; Says = @('Direct3D 12', '--uninstall no-such-install --quiet'); Arguments = @('--uninstall', 'no-such-install') }
    )) {
    # A dialog is small and plain, so it has few colors, and text is what makes up what it has.
    $opened = Open-And-Look $case.Program $case.Name $case.Title 2 3 -Arguments $(if ($case.Arguments) { $case.Arguments } else { @() })
    if ($opened) {
        $text = [WamWindow]::TextIn($script:LastWindow)
        Write-Host "  what its window says:`n$text"
        foreach ($words in $case.Says) {
            Check "$($case.Name): it says [$words]" ($text -match [regex]::Escape($words)) $text
        }
        Ask-To-Close $opened $case.Name 1
    }
}
Remove-Item Env:WGPU_BACKEND

Write-Host "`n=== With the environment naming a folder of Direct3D's code for these to run"
# A program that runs as the account these are started from can set these for it, and elevating keeps
# them. Direct3D loads D3D12Core.dll from there if asked for a newer version than Windows has, so what
# is in that folder would run with full rights, under this project's signature. What is planted here is
# a stand-in that only says it ran. It is not left loaded for long, so a list of what is loaded would miss it.
$plantedFolder = Join-Path $env:RUNNER_TEMP 'wam-planted-d3d'
New-Item -ItemType Directory -Force -Path $plantedFolder | Out-Null
Copy-Item -LiteralPath (Join-Path $Planted 'D3D12Core.dll') -Destination (Join-Path $plantedFolder 'D3D12Core.dll') -Force
$marker = Join-Path $env:RUNNER_TEMP 'wam-planted-ran.txt'
$env:WAM_PLANTED_MARKER = $marker
$env:WGPU_DX12_AGILITY_SDK_PATH = $plantedFolder
$env:WGPU_DX12_AGILITY_SDK_VERSION = '999'
foreach ($case in @(
        @{ Program = $App; Name = 'The app, told to run planted code'; Title = 'WinAppMigrator'; Settle = 20; Colors = 32; Arguments = @() },
        @{ Program = $installer; Name = 'The installer, told to run planted code'; Title = "Install $($expected.app.name)"; Settle = 15; Colors = 32; Arguments = @() },
        @{ Program = $installer; Name = 'The uninstaller, told to run planted code'; Title = 'Uninstall'; Settle = 8; Colors = 8; Arguments = @('--uninstall', 'no-such-install') }
    )) {
    Remove-Item -LiteralPath $marker -ErrorAction SilentlyContinue
    $opened = Open-And-Look $case.Program $case.Name $case.Title $case.Colors $case.Settle -Arguments $case.Arguments
    $ran = if (Test-Path -LiteralPath $marker) { Get-Content -Raw -LiteralPath $marker } else { '' }
    Check "$($case.Name): none of it ran" ($ran -eq '') $ran
    if ($opened) {
        # Listing another program's modules can fail where it runs under emulation.
        try {
            $listed = @($opened.Modules | ForEach-Object { $_.FileName })
            Write-Host "  it has $($listed.Count) modules loaded, and its D3D12Core is: $(@($listed | Where-Object { $_ -match 'D3D12Core' }) -join ', ')"
            $fromThere = @($listed | Where-Object { $_ -like "$plantedFolder*" })
            Check "$($case.Name): none of what it has loaded is from that folder" ($fromThere.Count -eq 0) ($fromThere -join ', ')
        } catch {
            Write-Host "  its modules could not be listed: $($_.Exception.Message)"
        }
        Ask-To-Close $opened $case.Name 0
    }
}
Remove-Item Env:WGPU_DX12_AGILITY_SDK_PATH
Remove-Item Env:WGPU_DX12_AGILITY_SDK_VERSION
Remove-Item Env:WAM_PLANTED_MARKER

Write-Host "`nSummary: $($script:failures.Count) failures."
if ($script:failures.Count -gt 0) {
    $script:failures | ForEach-Object { Write-Host "  $_" }
    exit 1
}
exit 0
