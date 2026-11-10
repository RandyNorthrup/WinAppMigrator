# Whether the app carries this stub, byte for byte. A size says nothing: the app is bigger than the stub
# with no stub in it at all, and an app without one builds no installers. `-Name client` checks the
# client the app saves for an old PC the same way.
param(
    [Parameter(Mandatory)][string]$App,
    [Parameter(Mandatory)][string]$Stub,
    [string]$Name = 'stub'
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
    throw "The $Name is not inside the app: no $($stubBytes.Length) bytes of $($appBytes.Length) match it."
}
$built = [BitConverter]::ToString([Security.Cryptography.SHA256]::Create().ComputeHash($stubBytes))
Write-Host "The app holds the $Name at byte $place of $($appBytes.Length): SHA-256 $built"
