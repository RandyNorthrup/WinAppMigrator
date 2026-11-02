# Code signing

## What is signed

| File | Signed? | Why |
| --- | --- | --- |
| `winappmigrator.exe`, the app | Yes, by the release workflow | It is what people download. The UAC prompt shows the publisher, Randy Northrup. |
| `winappmigrator-installer.exe`, the stub | **Never** | See below. It is embedded in the app and never published alone. |
| An installer you build | Only if you sign it yourself | You are its publisher, not this project. |

## Why the stub must never be signed

The stub reinstalls whatever package it carries, elevated, with no check on who made that
package. Windows leaves its own certificate table out of the signature's hash, so anyone
holding a signed stub could hide a package inside that table and the signature would still
read as valid. This is [CVE-2013-3900][cve]; Microsoft's fix is opt-in and off by default.
Confirmed on Windows 11: 4 KB added this way still reported `Valid`.

The result would be malware showing "Verified publisher: Randy Northrup" at the UAC prompt.
The certificate profile is shared with other projects, so revoking it would hit them too.

Three independent layers stop this:

1. **The release workflow** fails if the stub carries a signature, and signs only the app.
2. **Building refuses a signed stub** (`refuse_signed_stub` in `archive.rs`).
3. **The stub only trusts a package a signature would cover.** It reads the package from
   directly before the certificate table, never from inside it, and refuses any data after
   it. So even a stub signed by mistake could not be abused this way.

## Signing an installer you built

Sign the finished installer. The signature then covers that one package and nothing else,
and any change to the file invalidates it.

```powershell
az login
.\scripts\sign.ps1 'D:\Out\Example_installer.exe'
```

The script's defaults are this project's account, and work only for someone holding the
Signer role on it. Anyone else passes their own `-Endpoint`, `-Account`, and
`-CertificateProfile`.

Sign only installers you built from software you are entitled to redistribute, because your
name goes on them. Do not offer signing as a service to others.

An installer's window says who signed it, or "Not signed: publisher unknown". Signing is the only
thing that proves where one came from. The SHA-256 in its footer is checked before anything is read,
which catches a truncated copy or an edited manifest, but whoever edits a package can update that too.

## How release signing is set up

Signing uses [Azure Artifact Signing][signing] with no stored password or certificate:

- **Azure app registration:** `WinAppMigrator release signing (GitHub Actions)`, client ID
  `29291d6a-9068-4ccb-b193-1456129e05af`. It has no secrets and no certificates.
- **Federated credential:** Azure accepts GitHub's OIDC token only for the subject
  `repo:RandyNorthrup@122551425/WinAppMigrator@1267867179:environment:release`. Those are
  immutable owner and repo IDs, so a renamed or re-created repository does not inherit it.
- **Role:** `Artifact Signing Certificate Profile Signer`, scoped to the one certificate
  profile `SAKUtility` in account `scnetsolutions`, resource group `SAK`.
- **GitHub environment `release`:** deployable from `v*` tags only. It holds the
  `AZURE_CLIENT_ID` and `AZURE_TENANT_ID` secrets, which identify but do not authenticate.

Cut a release by pushing a `v*` tag. To retry a run that failed before publishing, start the
workflow from that tag. If the release already exists, delete it first.

Check the exe or the zip:

```powershell
gh attestation verify .\WinAppMigrator-v1.2.3.exe --repo RandyNorthrup/WinAppMigrator
(Get-AuthenticodeSignature .\WinAppMigrator-v1.2.3.exe).SignerCertificate.Subject
```

## Other protections for signed, elevated executables

A signed program that runs elevated is a target for loading someone else's code under its
name, so both executables also:

- resolve imported DLLs from System32 only (`/DEPENDENTLOADFLAG:0x800`), and DLLs loaded at
  runtime likewise (`SetDefaultDllDirectories`, first thing in `main`);
- link the C runtime statically, removing `VCRUNTIME140.dll` from the search entirely;
- run Windows tools by absolute path, with the Windows directory read from the admin-only
  registry rather than from an environment variable the user controls. winget is not a
  Windows tool. `PATH` only holds a shortcut to it in a folder the user can write to, so the
  installer asks Windows where its package is installed and checks that file's signature;
- start PowerShell with its module path set to the Windows folder's, and without .NET
  profiling, because both are otherwise taken from the user's environment;
- check that a prerequisite's installer is validly signed by Microsoft when it is downloaded
  and again just before running it;
- write the embedded stub straight into the installer, never through `%TEMP%`.

[cve]: https://msrc.microsoft.com/update-guide/vulnerability/CVE-2013-3900
[signing]: https://learn.microsoft.com/azure/artifact-signing/
