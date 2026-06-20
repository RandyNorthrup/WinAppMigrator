param(
    [Parameter(Mandatory = $true)]
    [string] $Exe,

    [string] $WinAppMigratorExe = ".\target\release\winappmigrator.exe",

    [string] $OutRoot = ".\target\wam-perf",

    [int] $PackageSecondsTarget = 120,

    [int] $VerifySecondsTarget = 30,

    [int] $RestoreDryRunSecondsTarget = 60
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-TimedGate {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Name,

        [Parameter(Mandatory = $true)]
        [int] $TargetSeconds,

        [Parameter(Mandatory = $true)]
        [scriptblock] $Script
    )

    $elapsed = Measure-Command { & $Script }
    $seconds = [Math]::Round($elapsed.TotalSeconds, 2)
    Write-Host "$Name completed in $seconds seconds. Target: <= $TargetSeconds seconds."
    if ($elapsed.TotalSeconds -gt $TargetSeconds) {
        throw "$Name exceeded performance target."
    }
}

$tool = Resolve-Path -LiteralPath $WinAppMigratorExe
$exePath = Resolve-Path -LiteralPath $Exe
$root = [System.IO.Path]::GetFullPath($OutRoot)
$packageRoot = Join-Path $root "package"
$restoreRoot = Join-Path $root "restore"

if (Test-Path $root) {
    Remove-Item -LiteralPath $root -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $root | Out-Null

Invoke-TimedGate "Package" $PackageSecondsTarget {
    & $tool package --exe $exePath --out $packageRoot --archive zip
}

Invoke-TimedGate "Verify package" $VerifySecondsTarget {
    & $tool verify --package $packageRoot
}

Invoke-TimedGate "Restore dry-run" $RestoreDryRunSecondsTarget {
    & $tool restore --package "$packageRoot.zip" --target-root $restoreRoot --dry-run
}
