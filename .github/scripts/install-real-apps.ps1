# The publishers' own installers of the real apps a move is tried with, each an exact version whose
# SHA-256 has to match, run silently on this runner, then asked the same as the runners they are moved
# to. What fails here fails with no help from this project. See
# crates/winappmigrator-installer/tests/elevated_real_apps.rs.
#
# Run only by the job that builds their installers, on a runner that is thrown away afterward.
param([Parameter(Mandatory)][string]$Output)

$ErrorActionPreference = 'Stop'
$ProgressPreference = 'SilentlyContinue'
. (Join-Path $PSScriptRoot 'real-app-checks.ps1')
Require-ThrowawayRunner

$apps = (Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot 'real-apps.json') | ConvertFrom-Json).apps
$downloads = New-WorkFolder
New-Item -ItemType Directory -Force -Path $Output | Out-Null

foreach ($app in $apps) {
    Write-Host "`n=== $($app.name)"
    Remove-Preinstalled $app

    $file = Join-Path $downloads ([IO.Path]::GetFileName(([Uri]$app.url).AbsolutePath))
    Invoke-WebRequest -Uri $app.url -OutFile $file -MaximumRetryCount 5 -RetryIntervalSec 10
    $hash = Sha256-Of $file
    if ($hash -ne $app.sha256) { throw "$($app.url) is $hash, and $($app.sha256) was expected" }
    Write-Host "  $file has the SHA-256 it should: $hash"

    if ($file.EndsWith('.msi')) {
        $log = Join-Path $downloads "$($app.id).log"
        $process = Start-Process msiexec.exe -ArgumentList (@('/i', "`"$file`"") + $app.arguments + @('/l*v', "`"$log`"")) -Wait -PassThru
    } else {
        $process = Start-Process $file -ArgumentList $app.arguments -Wait -PassThru
    }
    Write-Host "  its installer exited with $($process.ExitCode)"
    if ($process.ExitCode -notin 0, 3010) { throw "$($app.name) did not install: $($process.ExitCode)" }

    $entries = @(Uninstall-Entries $app.name)
    foreach ($entry in $entries) { Write-Host "  now in Settings: $($entry.DisplayName), at $($entry.Key), location [$($entry.InstallLocation)]: $($entry.Uninstall)" }
    if ($entries.Count -ne 1) { throw "$($entries.Count) entries start with $($app.name)" }
    if (-not (Test-Path -LiteralPath $app.installed_at)) { throw "$($app.name) is not at $($app.installed_at)" }
}

Write-Host "`n=== What the publishers' own installers give, with no reboot"
$baseline = [ordered]@{}
foreach ($app in $apps) {
    $results = [ordered]@{}
    foreach ($check in $script:Checks[$app.id]) {
        $problem = Check-Result $check $app.installed_at
        $results[$check.Name] = $problem
        Write-Host ("  {0,-16} {1,-8} {2}{3}" -f $app.id, $(if ($problem) { 'FAILED' } else { 'ok' }), $check.Name, $(if ($problem) { ": $problem" }))
    }
    $baseline[$app.id] = $results
}
# As they would be on a PC in use, so that a build has to read what is running.
foreach ($name in 'mosquitto', 'windows_exporter') {
    Start-Service -Name $name -ErrorAction SilentlyContinue
    Write-Host "  $name is $((Get-Service -Name $name).Status), $((Get-Service -Name $name).StartType)"
}
$baseline | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $Output 'baseline.json')
# Without the publisher's own installer doing it, the same failure later would say nothing of this project.
$failed = @($baseline.Values | ForEach-Object { $_.Values } | Where-Object { $_ })
if ($failed.Count -gt 0) {
    Write-Host "`n$($failed.Count) checks failed with the publishers' own installers."
    exit 1
}
exit 0
