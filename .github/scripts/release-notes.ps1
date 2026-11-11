# Writes a release's notes: what the app is, from .github/release-notes.md, and what is new in this version, from the
# paragraph under its heading in CHANGELOG.md, before its first `###`. A version with no such paragraph stops the release.
param(
    [Parameter(Mandatory)][string]$Version,
    [Parameter(Mandatory)][string]$Out
)
$ErrorActionPreference = 'Stop'
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$lines = Get-Content -LiteralPath (Join-Path $root 'CHANGELOG.md') -Encoding utf8
$heading = '^## ' + [regex]::Escape($Version) + '(\s|$)'
$start = -1
for ($i = 0; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match $heading) { $start = $i + 1; break }
}
if ($start -lt 0) { throw "CHANGELOG.md has no section for $Version." }
$summary = New-Object System.Collections.Generic.List[string]
for ($i = $start; $i -lt $lines.Count; $i++) {
    if ($lines[$i] -match '^##') { break }
    $summary.Add($lines[$i])
}
# One paragraph, unwrapped, as GitHub would show it anyway.
$text = (($summary -join "`n").Trim() -split '\s*\n\s*') -join ' '
if (-not $text) { throw "CHANGELOG.md's $Version section has no summary paragraph before its first ###." }
$template = Get-Content -LiteralPath (Join-Path $root '.github/release-notes.md') -Raw -Encoding utf8
$notes = $template.Replace('{{VERSION}}', $Version).Replace('{{SUMMARY}}', $text)
[IO.File]::WriteAllText($Out, $notes, [Text.UTF8Encoding]::new($false))
