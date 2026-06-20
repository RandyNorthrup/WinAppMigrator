param(
    [string] $WinAppMigratorExe = ".\target\release\winappmigrator.exe",

    [string] $WorkRoot = ".\target\wam-user-e2e",

    [switch] $KeepUserArtifacts,

    [switch] $KeepRestoredFiles
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

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

function Invoke-Native {
    param(
        [Parameter(Mandatory = $true)]
        [string] $FilePath,

        [Parameter(ValueFromRemainingArguments = $true)]
        [string[]] $Arguments
    )

    & $FilePath @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$FilePath failed with exit code $LASTEXITCODE. Args: $($Arguments -join ' ')"
    }
}

function Remove-TestTask {
    param(
        [Parameter(Mandatory = $true)]
        [string] $TaskName,

        [Parameter(Mandatory = $true)]
        [string] $TaskPath
    )

    $existing = Get-ScheduledTask -TaskName $TaskName -TaskPath $TaskPath -ErrorAction SilentlyContinue
    if ($null -ne $existing) {
        Unregister-ScheduledTask -TaskName $TaskName -TaskPath $TaskPath -Confirm:$false
    }
}

function Remove-TestUserArtifacts {
    param(
        [Parameter(Mandatory = $true)]
        [string] $TaskName,

        [Parameter(Mandatory = $true)]
        [string] $TaskPath,

        [Parameter(Mandatory = $true)]
        [string] $RegistryKeyPath,

        [Parameter(Mandatory = $true)]
        [string] $ShortcutPath,

        [Parameter(Mandatory = $true)]
        [string] $EnvironmentName
    )

    Remove-TestTask -TaskName $TaskName -TaskPath $TaskPath

    if (Test-Path -LiteralPath $RegistryKeyPath) {
        Remove-Item -LiteralPath $RegistryKeyPath -Recurse -Force
    }
    if (Test-Path -LiteralPath $ShortcutPath) {
        Remove-Item -LiteralPath $ShortcutPath -Force
    }

    $envKey = "HKCU:\Environment"
    if ((Get-ItemProperty -LiteralPath $envKey -Name $EnvironmentName -ErrorAction SilentlyContinue) -ne $null) {
        Remove-ItemProperty -LiteralPath $envKey -Name $EnvironmentName -Force
    }
}

function Assert-VerificationPassed {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Path
    )

    $report = Get-Content -Raw -LiteralPath $Path | ConvertFrom-Json
    if (-not $report.package_valid) {
        throw "Package verification failed. Report: $Path"
    }
    if ($null -ne $report.restore_valid -and -not $report.restore_valid) {
        throw "Restore verification failed. Report: $Path"
    }
    if ($report.errors.Count -gt 0) {
        throw "Verification reported $($report.errors.Count) errors. Report: $Path"
    }
}

$tool = Resolve-Path -LiteralPath $WinAppMigratorExe
$workRootFull = [System.IO.Path]::GetFullPath($WorkRoot)
$id = "WAM-USER-E2E-{0}" -f ([Guid]::NewGuid().ToString("N").Substring(0, 8))
$displayName = "WinAppMigrator Disposable User E2E $id"
$sourceRoot = Join-Path $env:LOCALAPPDATA $id
$sourceExe = Join-Path $sourceRoot "$id.exe"
$targetRoot = Join-Path $workRootFull "restored"
$packageRoot = Join-Path $workRootFull "package"
$packageZip = "$packageRoot.zip"
$taskName = "WinAppMigratorUserE2E_$($id.Replace('-', '_'))"
$taskPath = "\WinAppMigratorUserE2E\"
$registryKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$taskName"
$startMenu = [Environment]::GetFolderPath("StartMenu")
$shortcutPath = Join-Path $startMenu "Programs\$displayName.lnk"
$environmentName = "WINAPPMIGRATOR_USER_E2E_$($id.Replace('-', '_'))"
$userArtifactsCreated = $false
$restoreApplied = $false

if (Test-Path -LiteralPath $workRootFull) {
    Remove-Item -LiteralPath $workRootFull -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $workRootFull | Out-Null

try {
    Invoke-Step "Create disposable user-scoped fixture" {
        New-Item -ItemType Directory -Force -Path $sourceRoot | Out-Null
        Copy-Item -LiteralPath $tool -Destination $sourceExe -Force

        New-Item -Path $registryKey -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "DisplayName" -Value $displayName -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "DisplayVersion" -Value "1.0.0" -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "Publisher" -Value "WinAppMigrator E2E" -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "InstallLocation" -Value $sourceRoot -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "DisplayIcon" -Value "`"$sourceExe`",0" -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "UninstallString" -Value "`"$sourceExe`" --help" -PropertyType String -Force | Out-Null

        $action = New-ScheduledTaskAction -Execute $sourceExe -Argument "--help" -WorkingDirectory $sourceRoot
        $trigger = New-ScheduledTaskTrigger -Once -At (Get-Date).AddYears(5)
        $principal = New-ScheduledTaskPrincipal -UserId ([Security.Principal.WindowsIdentity]::GetCurrent().Name) -LogonType Interactive -RunLevel Limited
        Register-ScheduledTask -TaskName $taskName -TaskPath $taskPath -Action $action -Trigger $trigger -Principal $principal -Description "Disposable WinAppMigrator user-scope E2E task" -Force | Out-Null

        $shell = New-Object -ComObject WScript.Shell
        $shortcut = $shell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = $sourceExe
        $shortcut.Arguments = "--help"
        $shortcut.WorkingDirectory = $sourceRoot
        $shortcut.IconLocation = "$sourceExe,0"
        $shortcut.Save()

        New-ItemProperty -LiteralPath "HKCU:\Environment" -Name $environmentName -Value $sourceRoot -PropertyType ExpandString -Force | Out-Null
        $script:userArtifactsCreated = $true
    }

    Invoke-Step "Build user-scoped package by installed app name" {
        Invoke-Native $tool package --name $displayName --yes --non-interactive --out $packageRoot --archive zip
    }

    Invoke-Step "Verify package before source removal" {
        Invoke-Native $tool verify --package $packageRoot
        Assert-VerificationPassed -Path (Join-Path $packageRoot "verification_report.json")
    }

    Invoke-Step "Remove source user-scoped fixture before restore" {
        Remove-TestUserArtifacts `
            -TaskName $taskName `
            -TaskPath $taskPath `
            -RegistryKeyPath $registryKey `
            -ShortcutPath $shortcutPath `
            -EnvironmentName $environmentName
        if (Test-Path -LiteralPath $sourceRoot) {
            Remove-Item -LiteralPath $sourceRoot -Recurse -Force
        }
    }

    Invoke-Step "Restore dry-run from zip" {
        Invoke-Native $tool restore --package $packageZip --target-root $targetRoot --dry-run
    }

    Invoke-Step "Restore apply from zip" {
        Invoke-Native $tool restore --package $packageZip --target-root $targetRoot --apply --force
        $script:restoreApplied = $true
    }

    Invoke-Step "Verify restored user-scoped integrations" {
        Invoke-Native $tool verify --package $packageZip --target-root $targetRoot
        Assert-VerificationPassed -Path (Join-Path $workRootFull "verification_report.json")
    }

    Write-Host "Disposable user-scope E2E passed."
    Write-Host "Evidence directory: $workRootFull"
}
finally {
    if ($userArtifactsCreated -and -not $KeepUserArtifacts) {
        Remove-TestUserArtifacts `
            -TaskName $taskName `
            -TaskPath $taskPath `
            -RegistryKeyPath $registryKey `
            -ShortcutPath $shortcutPath `
            -EnvironmentName $environmentName
    }

    if ((Test-Path -LiteralPath $sourceRoot) -and -not $KeepUserArtifacts) {
        Remove-Item -LiteralPath $sourceRoot -Recurse -Force
    }

    if ($restoreApplied -and (Test-Path -LiteralPath $targetRoot) -and -not $KeepRestoredFiles) {
        Remove-Item -LiteralPath $targetRoot -Recurse -Force
    }
}
