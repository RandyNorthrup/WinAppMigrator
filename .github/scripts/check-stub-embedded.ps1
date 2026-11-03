# Whether the app carries this stub, byte for byte. A size says nothing: the app is bigger than the stub
# with no stub in it at all, and an app without one builds no installers.
param(
    [Parameter(Mandatory)][string]$App,
    [Parameter(Mandatory)][string]$Stub
)

$ErrorActionPreference = 'Stop'

Add-Type -TypeDefinition @'
using System;
public static class WamBytes {
    public static int FirstPlaceOf(byte[] whole, byte[] part) {
        return new ReadOnlySpan<byte>(whole).IndexOf(new ReadOnlySpan<byte>(part));
    }
}
'@

$appBytes = [IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $App))
$stubBytes = [IO.File]::ReadAllBytes((Resolve-Path -LiteralPath $Stub))
if ($stubBytes.Length -eq 0) { throw "$Stub is empty." }

$place = [WamBytes]::FirstPlaceOf($appBytes, $stubBytes)
if ($place -lt 0) {
    throw "The stub is not inside the app: no $($stubBytes.Length) bytes of $($appBytes.Length) match it."
}
$sha = [Security.Cryptography.SHA256]::Create()
$inTheApp = [BitConverter]::ToString($sha.ComputeHash($appBytes, $place, $stubBytes.Length))
$built = [BitConverter]::ToString($sha.ComputeHash($stubBytes))
if ($inTheApp -ne $built) { throw "What is inside the app is not the stub: $inTheApp, and the stub is $built." }
Write-Host "The app holds the stub at byte $place of $($appBytes.Length): SHA-256 $built"
