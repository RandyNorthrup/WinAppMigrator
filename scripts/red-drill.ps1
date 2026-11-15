# A red drill: prove a test catches the defect it is there for. In a disposable git worktree (never this one), it runs
# the test command and needs it to pass, puts one defect into one file, needs the same command to fail with output
# holding the expected text (the whole line it is on is recorded), puts the file back byte for byte, and needs the
# command to pass again. It writes what happened to a log and a JSON summary, and exits 0 only if all of that held.
#
#   scripts/red-drill.ps1 -Worktree <path> -File <relative path> -Find <text> -Replace <text> `
#       -Command <command line> -Expect <text> -Out <folder> -Name <drill name> [-Setup <command>] [-Cleanup <command>]
#
# Make the worktree with `git worktree add --detach <path> HEAD`, and give it its own CARGO_TARGET_DIR so that drills
# never touch this repository's build. -Setup makes a fixture the drill then changes, such as a second catalog, and
# -Cleanup removes it.
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string] $Worktree,
    [Parameter(Mandatory)][string] $File,
    [Parameter(Mandatory)][string] $Find,
    [Parameter(Mandatory)][AllowEmptyString()][string] $Replace,
    [Parameter(Mandatory)][string] $Command,
    [Parameter(Mandatory)][string] $Expect,
    [Parameter(Mandatory)][string] $Out,
    [Parameter(Mandatory)][string] $Name,
    [string] $Setup = '',
    [string] $Cleanup = ''
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repository = (git -C $PSScriptRoot rev-parse --show-toplevel).Trim()
$worktreeRoot = (Resolve-Path -LiteralPath $Worktree).Path
if ([IO.Path]::GetFullPath($worktreeRoot).TrimEnd('\', '/') -ieq [IO.Path]::GetFullPath($repository).TrimEnd('\', '/')) {
    throw 'A drill runs in a disposable worktree, never in the repository itself.'
}
New-Item -ItemType Directory -Force -Path $Out | Out-Null
$log = Join-Path $Out "$Name.log"
Set-Content -LiteralPath $log -Encoding utf8 -Value @"
drill $Name
file $File
mutation: '$Find' -> '$Replace'
command: $Command
expect: $Expect
"@

function Get-Digest([string] $Path) { (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant() }

function Invoke-InWorktree([string] $Line) {
    Push-Location -LiteralPath $worktreeRoot
    try {
        $output = & pwsh -NoProfile -Command "$Line 2>&1 | Out-String -Width 400; exit `$LASTEXITCODE"
        return [pscustomobject]@{ Code = $LASTEXITCODE; Text = ($output | Out-String) }
    } finally {
        Pop-Location
    }
}

function Invoke-Step([string] $Label) {
    $result = Invoke-InWorktree $Command
    Add-Content -LiteralPath $log -Encoding utf8 -Value "===== $Label`n$($result.Text)`nexit $($result.Code)"
    return $result
}

if ($Setup) {
    $made = Invoke-InWorktree $Setup
    Add-Content -LiteralPath $log -Encoding utf8 -Value "===== setup`n$($made.Text)`nexit $($made.Code)"
    if ($made.Code -ne 0) { throw "setup failed (exit $($made.Code))" }
}
$target = Join-Path $worktreeRoot $File
$original = [IO.File]::ReadAllBytes($target)
$before = Get-Digest $target
$summary = [ordered]@{
    name = $Name; file = ($File -replace '\\', '/'); mutation = "$Find -> $Replace"; command = $Command
    expected_diagnostic = $Expect; before = $before; mutated = $null; after = $null
    baseline_exit = $null; mutated_exit = $null; restored_exit = $null; observed_diagnostic = $null
    failure = $null; passed = $false
}
$failure = $null
try {
    $baseline = Invoke-Step 'baseline'
    $summary.baseline_exit = $baseline.Code
    if ($baseline.Code -ne 0) { throw "the baseline does not pass (exit $($baseline.Code))" }

    $text = [Text.Encoding]::UTF8.GetString($original)
    $count = ([regex]::Matches($text, [regex]::Escape($Find))).Count
    if ($count -ne 1) { throw "the text to change appears $count times, not once" }
    [IO.File]::WriteAllText($target, $text.Replace($Find, $Replace), [Text.UTF8Encoding]::new($false))
    $summary.mutated = Get-Digest $target

    $mutated = Invoke-Step 'mutated'
    $summary.mutated_exit = $mutated.Code
    $at = $mutated.Text.IndexOf($Expect, [StringComparison]::Ordinal)
    if ($at -ge 0) {
        $lineStart = $mutated.Text.LastIndexOf("`n", $at) + 1
        $lineEnd = $mutated.Text.IndexOf("`n", $at)
        if ($lineEnd -lt 0) { $lineEnd = $mutated.Text.Length }
        $summary.observed_diagnostic = $mutated.Text.Substring($lineStart, $lineEnd - $lineStart).Trim()
    }
    if ($mutated.Code -eq 0) { throw 'the defect was not caught: the command passed' }
    if ($at -lt 0) { throw 'the command failed, but not with the expected message' }
} catch {
    $failure = $_.Exception.Message
} finally {
    [IO.File]::WriteAllBytes($target, $original)
    $summary.after = Get-Digest $target
}
if (-not $failure -and $summary.after -ne $before) { $failure = 'the file was not put back as it was' }
if (-not $failure) {
    $restored = Invoke-Step 'restored'
    $summary.restored_exit = $restored.Code
    if ($restored.Code -ne 0) { $failure = "the restored file does not pass (exit $($restored.Code))" }
}
if ($Cleanup) {
    $removed = Invoke-InWorktree $Cleanup
    Add-Content -LiteralPath $log -Encoding utf8 -Value "===== cleanup`n$($removed.Text)`nexit $($removed.Code)"
}
$summary.failure = $failure
$summary.passed = -not $failure
$summary | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $Out "$Name.json") -Encoding utf8
if ($failure) {
    Add-Content -LiteralPath $log -Encoding utf8 -Value "DRILL FAILED: $failure"
    Write-Error "drill $Name failed: $failure"
    exit 1
}
"drill $Name passed: baseline 0, mutated $($summary.mutated_exit) with '$($summary.observed_diagnostic)', restored 0"
