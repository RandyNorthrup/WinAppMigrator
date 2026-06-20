param(
    [string] $WinAppMigratorExe = ".\target\release\winappmigrator.exe",

    [string] $WorkRoot = ".\target\wam-elevated-machine-e2e",

    [switch] $KeepMachineArtifacts,

    [switch] $KeepRestoredFiles
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

function Remove-TestService {
    param([Parameter(Mandatory = $true)][string] $Name)

    $existing = Get-Service -Name $Name -ErrorAction SilentlyContinue
    if ($null -ne $existing) {
        if ($existing.Status -ne "Stopped") {
            Stop-Service -Name $Name -Force -ErrorAction SilentlyContinue
        }
        & sc.exe delete $Name | Out-Null
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

function Remove-TestMachineArtifacts {
    param(
        [Parameter(Mandatory = $true)]
        [string] $ServiceName,

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

    Remove-TestService -Name $ServiceName
    Remove-TestTask -TaskName $TaskName -TaskPath $TaskPath

    if (Test-Path -LiteralPath $RegistryKeyPath) {
        Remove-Item -LiteralPath $RegistryKeyPath -Recurse -Force
    }
    if (Test-Path -LiteralPath $ShortcutPath) {
        Remove-Item -LiteralPath $ShortcutPath -Force
    }

    $envKey = "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment"
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

if (-not (Test-IsAdministrator)) {
    throw "Disposable machine-scope E2E requires an elevated PowerShell session."
}

$tool = Resolve-Path -LiteralPath $WinAppMigratorExe
$workRootFull = [System.IO.Path]::GetFullPath($WorkRoot)
$id = "WAM-E2E-{0}" -f ([Guid]::NewGuid().ToString("N").Substring(0, 8))
$displayName = "WinAppMigrator Disposable Machine E2E $id"
$sourceRoot = Join-Path $env:ProgramFiles $id
$sourceExe = Join-Path $sourceRoot "$id.exe"
$targetRoot = Join-Path $workRootFull "restored"
$packageRoot = Join-Path $workRootFull "package"
$packageZip = "$packageRoot.zip"
$serviceName = "WinAppMigratorE2E_$($id.Replace('-', '_'))"
$taskName = $serviceName
$taskPath = "\WinAppMigratorE2E\"
$registryKey = "HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\$serviceName"
$commonStartMenu = [Environment]::GetFolderPath("CommonStartMenu")
$shortcutPath = Join-Path $commonStartMenu "Programs\$displayName.lnk"
$environmentName = "WINAPPMIGRATOR_E2E_$($id.Replace('-', '_'))"
$machineArtifactsCreated = $false
$restoreApplied = $false

if (Test-Path -LiteralPath $workRootFull) {
    Remove-Item -LiteralPath $workRootFull -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $workRootFull | Out-Null

try {
    Invoke-Step "Create disposable machine-scoped fixture" {
        New-Item -ItemType Directory -Force -Path $sourceRoot | Out-Null
        Copy-Item -LiteralPath $tool -Destination $sourceExe -Force

        $script:machineArtifactsCreated = $true
        New-Item -Path $registryKey -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "DisplayName" -Value $displayName -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "DisplayVersion" -Value "1.0.0" -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "Publisher" -Value "WinAppMigrator E2E" -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "InstallLocation" -Value $sourceRoot -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "DisplayIcon" -Value "`"$sourceExe`",0" -PropertyType String -Force | Out-Null
        New-ItemProperty -LiteralPath $registryKey -Name "UninstallString" -Value "`"$sourceExe`" --help" -PropertyType String -Force | Out-Null

        New-Service `
            -Name $serviceName `
            -BinaryPathName "`"$sourceExe`" --help" `
            -DisplayName $displayName `
            -StartupType Manual | Out-Null
        New-ItemProperty `
            -LiteralPath "HKLM:\SYSTEM\CurrentControlSet\Services\$serviceName" `
            -Name "Description" `
            -Value "Disposable WinAppMigrator machine-scope E2E service" `
            -PropertyType String `
            -Force | Out-Null

        $action = New-ScheduledTaskAction -Execute $sourceExe -Argument "--help" -WorkingDirectory $sourceRoot
        $trigger = New-ScheduledTaskTrigger -Once -At (Get-Date).AddYears(5)
        $principal = New-ScheduledTaskPrincipal -UserId "SYSTEM" -RunLevel Highest
        Register-ScheduledTask -TaskName $taskName -TaskPath $taskPath -Action $action -Trigger $trigger -Principal $principal -Description "Disposable WinAppMigrator machine-scope E2E task" -Force | Out-Null

        $shell = New-Object -ComObject WScript.Shell
        $shortcut = $shell.CreateShortcut($shortcutPath)
        $shortcut.TargetPath = $sourceExe
        $shortcut.Arguments = "--help"
        $shortcut.WorkingDirectory = $sourceRoot
        $shortcut.IconLocation = "$sourceExe,0"
        $shortcut.Save()

        $envKey = "HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment"
        New-ItemProperty -LiteralPath $envKey -Name $environmentName -Value $sourceRoot -PropertyType ExpandString -Force | Out-Null
    }

    Invoke-Step "Build machine-scoped package by installed app name" {
        Invoke-Native $tool package --name $displayName --yes --non-interactive --out $packageRoot --archive zip
    }

    Invoke-Step "Verify package before source removal" {
        Invoke-Native $tool verify --package $packageRoot
        Assert-VerificationPassed -Path (Join-Path $packageRoot "verification_report.json")
    }

    Invoke-Step "Remove source machine-scoped fixture before restore" {
        Remove-TestMachineArtifacts `
            -ServiceName $serviceName `
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

    Invoke-Step "Verify restored machine-scoped integrations" {
        Invoke-Native $tool verify --package $packageZip --target-root $targetRoot
        Assert-VerificationPassed -Path (Join-Path $workRootFull "verification_report.json")
    }

    Write-Host "Disposable machine-scope E2E passed."
    Write-Host "Evidence directory: $workRootFull"
}
finally {
    if ($machineArtifactsCreated -and -not $KeepMachineArtifacts) {
        Remove-TestMachineArtifacts `
            -ServiceName $serviceName `
            -TaskName $taskName `
            -TaskPath $taskPath `
            -RegistryKeyPath $registryKey `
            -ShortcutPath $shortcutPath `
            -EnvironmentName $environmentName
    }

    if ((Test-Path -LiteralPath $sourceRoot) -and -not $KeepMachineArtifacts) {
        Remove-Item -LiteralPath $sourceRoot -Recurse -Force
    }

    if ($restoreApplied -and (Test-Path -LiteralPath $targetRoot) -and -not $KeepRestoredFiles) {
        Remove-Item -LiteralPath $targetRoot -Recurse -Force
    }
}
