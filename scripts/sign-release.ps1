param(
    [Parameter(Mandatory = $true)]
    [string[]] $Path,

    [Parameter(Mandatory = $true)]
    [string] $CertificatePath,

    [Parameter(Mandatory = $true)]
    [string] $CertificatePassword,

    [string] $TimestampUrl = "http://timestamp.digicert.com",

    [string] $DigestAlgorithm = "SHA256"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Find-SignTool {
    $command = Get-Command signtool.exe -ErrorAction SilentlyContinue
    if ($null -ne $command) {
        return $command.Source
    }

    $kitsRoot = "${env:ProgramFiles(x86)}\Windows Kits\10\bin"
    if (Test-Path $kitsRoot) {
        $candidate = Get-ChildItem -Path $kitsRoot -Recurse -Filter signtool.exe |
            Where-Object { $_.FullName -match '\\x64\\signtool.exe$' } |
            Sort-Object FullName -Descending |
            Select-Object -First 1
        if ($null -ne $candidate) {
            return $candidate.FullName
        }
    }

    throw "signtool.exe was not found."
}

$signTool = Find-SignTool
$cert = Resolve-Path -LiteralPath $CertificatePath

foreach ($item in $Path) {
    $target = Resolve-Path -LiteralPath $item
    & $signTool sign `
        /f $cert `
        /p $CertificatePassword `
        /fd $DigestAlgorithm `
        /tr $TimestampUrl `
        /td $DigestAlgorithm `
        $target

    & $signTool verify /pa /v $target
}
