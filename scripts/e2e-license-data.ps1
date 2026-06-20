param(
    [string] $WinAppMigratorExe = ".\target\release\winappmigrator.exe",

    [string] $WorkRoot = ".\target\wam-license-e2e",

    [switch] $KeepArtifacts
)

# Disposable, non-admin certification of optional license-data backup.
# Creates a self-contained fixture app directory with a plain license file, a
# machine-bound activation file, and a non-license file, then verifies the
# scanner detects the licenses (and only the licenses) and that `package
# --include-license-data` copies the license files into the package `license/`
# directory. Makes no system registry, service, task, or shortcut changes.

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Step {
    param(
        [Parameter(Mandatory = $true)] [string] $Name,
        [Parameter(Mandatory = $true)] [scriptblock] $Script
    )
    Write-Host "==> $Name"
    & $Script
}

function Invoke-Native {
    param(
        [Parameter(Mandatory = $true)] [string] $FilePath,
        [Parameter(ValueFromRemainingArguments = $true)] [string[]] $Arguments
    )
    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$FilePath failed with exit code $LASTEXITCODE. Args: $($Arguments -join ' ')"
    }
}

$tool = Resolve-Path -LiteralPath $WinAppMigratorExe
$workRootFull = [System.IO.Path]::GetFullPath($WorkRoot)
$appRoot = Join-Path $workRootFull "App"
$appExe = Join-Path $appRoot "DemoLicensedApp.exe"
$licenseFile = Join-Path $appRoot "product.lic"
$activationFile = Join-Path $appRoot "activation_001.v2c"
$decoyFile = Join-Path $appRoot "notes.txt"
$scanOut = Join-Path $workRootFull "scan"
$packageRoot = Join-Path $workRootFull "package"

if (Test-Path -LiteralPath $workRootFull) {
    Remove-Item -LiteralPath $workRootFull -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $appRoot | Out-Null

try {
    Invoke-Step "Create disposable licensed fixture" {
        Copy-Item -LiteralPath $tool -Destination $appExe -Force
        Set-Content -LiteralPath $licenseFile -Encoding Ascii `
            -Value "FEATURE demo democorp 1.0 permanent uncounted SIGN=ABC123"
        Set-Content -LiteralPath $activationFile -Encoding Ascii `
            -Value "Sentinel HASP V2C activation payload (binary placeholder)"
        Set-Content -LiteralPath $decoyFile -Encoding Ascii -Value "not a license"
    }

    Invoke-Step "Scan with license detection" {
        Invoke-Native $tool scan --exe $appExe --out $scanOut --include-license-data --quiet
        $report = Get-Content -Raw -LiteralPath (Join-Path $scanOut "scan_report.json") | ConvertFrom-Json
        $artifacts = @($report.license_artifacts)
        if ($artifacts.Count -lt 2) {
            throw "Expected at least 2 license artifacts (.lic, .v2c); found $($artifacts.Count)."
        }
        $paths = $artifacts | ForEach-Object { $_.path }
        if (-not ($paths -contains $licenseFile)) {
            throw "License file $licenseFile was not detected."
        }
        if (-not ($paths -contains $activationFile)) {
            throw "Activation file $activationFile was not detected."
        }
        if ($paths -contains $decoyFile) {
            throw "Non-license file $decoyFile was incorrectly flagged as license data."
        }
        $v2c = $artifacts | Where-Object { $_.path -eq $activationFile }
        if (-not $v2c.machine_bound) {
            throw ".v2c activation file should be reported as machine-bound."
        }
    }

    Invoke-Step "Package with license backup" {
        Invoke-Native $tool package --exe $appExe --out $packageRoot --include-license-data --quiet
        $manifest = Get-Content -Raw -LiteralPath (Join-Path $packageRoot "manifest.json") | ConvertFrom-Json
        $copied = @($manifest.license_artifacts_detected | Where-Object { $_.should_copy })
        if ($copied.Count -lt 2) {
            throw "Expected at least 2 copied license artifacts in the manifest; found $($copied.Count)."
        }
        $licenseDir = Join-Path $packageRoot "license"
        $copiedFiles = @(Get-ChildItem -LiteralPath $licenseDir -Recurse -File -ErrorAction SilentlyContinue)
        if (-not ($copiedFiles.Name -contains "product.lic")) {
            throw "product.lic was not copied into the package license/ directory."
        }
        if (-not ($copiedFiles.Name -contains "activation_001.v2c")) {
            throw "activation_001.v2c was not copied into the package license/ directory."
        }
    }

    Write-Host "License-data E2E passed."
    Write-Host "Evidence directory: $workRootFull"
}
finally {
    if (-not $KeepArtifacts -and (Test-Path -LiteralPath $workRootFull)) {
        Remove-Item -LiteralPath $workRootFull -Recurse -Force
    }
}
