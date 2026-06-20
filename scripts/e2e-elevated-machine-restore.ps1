param(
    [Parameter(Mandatory = $true)]
    [string] $Package,

    [Parameter(Mandatory = $true)]
    [string] $TargetRoot,

    [string] $WinAppMigratorExe = ".\target\release\winappmigrator.exe",

    [switch] $Apply,

    [switch] $Force
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Test-IsAdministrator {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = [Security.Principal.WindowsPrincipal]::new($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Name,

        [Parameter(Mandatory = $true)]
        [scriptblock] $Script
    )

    Write-Host "==> $Name"
    & $Script
}

$tool = Resolve-Path -LiteralPath $WinAppMigratorExe
$packagePath = Resolve-Path -LiteralPath $Package
$targetPath = [System.IO.Path]::GetFullPath($TargetRoot)

if ($Apply -and -not (Test-IsAdministrator)) {
    throw "Apply mode requires an elevated PowerShell session."
}

Invoke-Step "Verify package before restore" {
    & $tool verify --package $packagePath
}

Invoke-Step "Restore dry-run" {
    & $tool restore --package $packagePath --target-root $targetPath --dry-run
}

if ($Apply) {
    $applyArgs = @("restore", "--package", $packagePath, "--target-root", $targetPath, "--apply")
    if ($Force) {
        $applyArgs += "--force"
    }

    Invoke-Step "Restore apply" {
        & $tool @applyArgs
    }

    Invoke-Step "Verify restored target" {
        & $tool verify --package $packagePath --target-root $targetPath
    }
} else {
    Write-Host "Apply not requested. Re-run with -Apply from an elevated shell to perform machine-wide E2E apply."
}
