<#
.SYNOPSIS
    Signs a file with Azure Artifact Signing, using your own `az login`.

.DESCRIPTION
    Use it for an installer you built and intend to hand to someone else. The signature
    covers that exact installer, including its package. Never sign the bare installer
    stub: see docs/signing.md for why.

    Needs an account holding the "Artifact Signing Certificate Profile Signer" role, the
    .NET 8 runtime, and the module, which this script installs for the current user if
    it is missing.

.EXAMPLE
    .\scripts\sign.ps1 'D:\Out\Example_installer.exe'
#>
param(
    [Parameter(Mandatory = $true)]
    [string[]] $Path,

    [string] $Endpoint = 'https://wus.codesigning.azure.net/',
    [string] $Account = 'scnetsolutions',
    [string] $CertificateProfile = 'SAKUtility'
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# The version the signing action in release.yml uses. This runs under an account that can sign, so it takes that one
# version and no newer one that may turn up.
$moduleVersion = '0.1.8'
if (-not (Get-Module -ListAvailable ArtifactSigning | Where-Object Version -eq $moduleVersion)) {
    Install-Module -Name ArtifactSigning -RequiredVersion $moduleVersion -Repository PSGallery -Scope CurrentUser -Force -AcceptLicense
}
Import-Module ArtifactSigning -RequiredVersion $moduleVersion

$files = foreach ($item in $Path) { (Resolve-Path -LiteralPath $item).Path }

$arguments = @{
    Endpoint                           = $Endpoint
    CodeSigningAccountName             = $Account
    CertificateProfileName             = $CertificateProfile
    Files                              = $files -join ','
    FileDigest                         = 'SHA256'
    TimestampRfc3161                   = 'http://timestamp.acs.microsoft.com'
    TimestampDigest                    = 'SHA256'
    ExcludeEnvironmentCredential       = $true
    ExcludeWorkloadIdentityCredential  = $true
    ExcludeManagedIdentityCredential   = $true
    ExcludeSharedTokenCacheCredential  = $true
    ExcludeVisualStudioCredential      = $true
    ExcludeVisualStudioCodeCredential  = $true
    ExcludeAzurePowerShellCredential   = $true
    ExcludeAzureDeveloperCliCredential = $true
    ExcludeInteractiveBrowserCredential = $true
}
Invoke-ArtifactSigning @arguments

foreach ($file in $files) {
    $signature = Get-AuthenticodeSignature -LiteralPath $file
    if ($signature.Status -ne 'Valid') {
        throw "$file did not end up with a valid signature: $($signature.Status)"
    }
    Write-Host "Signed $file by $($signature.SignerCertificate.Subject)"
}
