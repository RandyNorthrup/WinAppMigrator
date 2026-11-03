# Opens the real programs, with real windows, on a runner with no graphics card. Everything else tests
# the screens with no window at all, which says nothing of whether one opens. It clicks nothing and types
# nothing: it waits for the window, takes a picture of what is on the screen, and asks it to close. An
# installer given no arguments waits at its first screen, so nothing is installed.
#
# It then opens each with graphics that cannot start, to see what a person is told.
#
# Run only by the job that downloads these, on a runner that is thrown away afterward.
param(
    [Parameter(Mandatory)][string]$App,
    [Parameter(Mandatory)][string]$Installers,
    [Parameter(Mandatory)][string]$Pictures
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

    delegate bool EachWindow(IntPtr window, IntPtr unused);
    [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr window, EachWindow each, IntPtr unused);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowTextW(IntPtr window, System.Text.StringBuilder text, int most);

    // What the windows in this one say, which is what a plain message box's text is.
    public static string TextIn(IntPtr window) {
        var all = new System.Text.StringBuilder();
        EnumChildWindows(window, (child, unused) => {
            var text = new System.Text.StringBuilder(4096);
            GetWindowTextW(child, text, text.Capacity);
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

# Whether a picture has anything drawn in it. A blank window is one color, or nearly.
function Is-Drawn($Colors, [int]$AtLeast) { return $Colors[0] -ge $AtLeast -and $Colors[1] -le 98 }

# Open a program, wait for its window, take a picture, ask it to close, and say what happened.
function Open-And-Look([string]$Program, [string]$Name, [string]$WindowTitle, [int]$MinimumColors, [int]$Settle) {
    Write-Host "`n--- $Name`: $Program"
    $process = Start-Process -FilePath $Program -PassThru
    $state = @{ Window = [IntPtr]::Zero }
    $opened = Wait-Until {
        $process.Refresh()
        if ($process.HasExited) { return $true }
        if ($process.MainWindowHandle -ne [IntPtr]::Zero -and [WamWindow]::IsWindowVisible($process.MainWindowHandle)) {
            $state.Window = $process.MainWindowHandle
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
    # It has to get as far as drawing, and what it shows first it has to go and find.
    Start-Sleep -Seconds $Settle
    $process.Refresh()
    Check "$Name`: it is still running" (-not $process.HasExited) "exit $(if ($process.HasExited) { $process.ExitCode })"
    if ($process.HasExited) { return $null }
    $state.Window = $process.MainWindowHandle
    $title = $process.MainWindowTitle
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

    $seen = @{}
    foreach ($way in 'on the screen', 'asked of the window') {
        try {
            $picture = if ($way -eq 'on the screen') { Picture-On-Screen $rect } else { Picture-Asked $state.Window $rect }
            $colors = Colors-Of $picture
            Write-Host "  $way`: $($colors[0]) colors, the commonest is $($colors[1])% of it"
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

function Ask-To-Close($Process, [string]$Name, [int]$ExpectedExit) {
    $Process.Refresh()
    if ($Process.HasExited) { return }
    $asked = $Process.CloseMainWindow()
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

Write-Host "`n=== The app, as it is released"
$opened = Open-And-Look $App 'The app' 'WinAppMigrator' 32 20
if ($opened) {
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
$opened = Open-And-Look $installer 'The installer' "Install $($expected.app.name)" 32 15
if ($opened) { Ask-To-Close $opened 'The installer' 0 }
Check 'The installer: it installed nothing' ((Test-Path -LiteralPath "C:\Program Files\$($expected.app.name)") -eq $before) ''

Write-Host "`n=== With graphics that cannot start, what a person is told"
# wgpu goes by this, and these programs have no renderer but Direct3D 12.
$env:WGPU_BACKEND = 'vulkan'
foreach ($case in @(
        @{ Program = $App; Name = 'The app, with no graphics'; Title = 'WinAppMigrator'; Says = @('Direct3D 12') },
        @{ Program = $installer; Name = 'The installer, with no graphics'; Title = 'WinAppMigrator Installer'; Says = @('Direct3D 12', '--apply', '--target-root') }
    )) {
    # A dialog is small and plain, so it has few colors, and text is what makes up what it has.
    $opened = Open-And-Look $case.Program $case.Name $case.Title 2 3
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

Write-Host "`nSummary: $($script:failures.Count) failures."
if ($script:failures.Count -gt 0) {
    $script:failures | ForEach-Object { Write-Host "  $_" }
    exit 1
}
exit 0
