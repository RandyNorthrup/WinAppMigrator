# Code signing

## What is signed

| File | Signed? | Why |
| --- | --- | --- |
| `winappmigrator.exe`, the app | Yes, by the release workflow | It is what people download. The UAC prompt shows the publisher, Randy Northrup. |
| `winappmigrator-installer.exe`, the stub | **Never** | See below. It is embedded in the app and never published alone. |
| `WinAppMigrator Client.exe`, the client | Yes, by the release workflow, before the app is built around it | It is run elevated on the old PC, where the UAC prompt then shows the publisher. See [why it may be](#why-the-client-may-be-signed). |
| An installer you build | Only if you sign it yourself | You are its publisher, not this project. |
| The loader of an installer in two files | Only if you sign it yourself | It is the `.exe` of that installer. Its footer holds the hash of its data file, so the signature covers that too. |
| The data file of an installer in two files | No | It is not a program, and has nothing to carry a signature. |

## Why the stub must never be signed

The stub reinstalls whatever package it carries, elevated, with no check on who made that
package. Windows leaves its own certificate table out of the signature's hash, so anyone
holding a signed stub could hide a package inside that table and the signature would still
read as valid. This is [CVE-2013-3900][cve]; Microsoft's fix is opt-in and off by default.
Confirmed on Windows 11: 4 KB added this way still reported `Valid`.

The result would be malware showing "Verified publisher: Randy Northrup" at the UAC prompt.
The certificate profile is shared with other projects, so revoking it would hit them too.

Three independent layers stop this:

1. **The release workflow** fails if the stub carries a signature, and signs only the app and the client. It also fails unless the app
   holds that stub byte for byte, and building an app for release fails with no stub to embed.
2. **Building refuses a signed stub** (`refuse_signed_stub` in `archive.rs`).
3. **The stub only trusts a package a signature would cover.** It reads the package, or the
   footer that vouches for a data file, from directly before the certificate table, never from
   inside it, and refuses any data after it. So even a stub signed by mistake could not be abused
   this way.

## Why the client may be signed

What made a signed stub dangerous is that it runs what its own file carries. The client reads nothing
from its own file: whatever is hidden in its certificate table is never looked at, so it still only
does what the client does. Its signature says the program is this project's, as the app's does.

What a signature does change is that someone could hand out the signed client beside a DLL of their
own, for Windows to load into it under this project's name. So, besides what every signed executable
here does (below):

- **It imports only DLLs Windows always loads from System32**: `kernel32`, `advapi32`, `ws2_32` and
  `ntdll`. `build-client.ps1` builds it, and its randomness for pairing comes through `RtlGenRandom` in
  `advapi32` rather than `bcrypt.dll`, which is not one of them, as Windows 7 ignores
  `/DEPENDENTLOADFLAG`.
- **It runs from a folder only an administrator can change.** Windows loads some DLLs only when first
  needed, by name, looking beside the program first whatever it asks: on 32-bit Windows 7,
  `RtlGenRandom` loads `cryptbase.dll` that way, and a copy planted beside the client was loaded into it.
  So the client first copies itself into `%ProgramFiles%\WinAppMigrator Client` and runs from there,
  having done nothing that loads a DLL. Tried on Windows 7, 32-bit and 64-bit, with Windows' own DLLs
  planted beside it: none were loaded, before the copy or after, through pairing and a whole build.
- **Every Windows tool it starts gets an environment of Windows' own**, not the person's, as below.

The release workflow signs it in a job of its own, which runs nothing from the repository or crates.io,
as it does the app, then checks its signature and that the app carries it byte for byte. See
[how release signing is set up](#how-release-signing-is-set-up).

## Signing an installer you built

Sign the finished installer. The signature then covers that one package and nothing else,
and any change to the file invalidates it.

```powershell
az login
.\scripts\sign.ps1 'D:\Out\Example_installer.exe'
```

The script's defaults are this project's account, and work only for someone holding the
Signer role on it. Anyone else passes their own `-Endpoint`, `-Account`, and
`-CertificateProfile`. It needs the .NET 8 runtime, and installs the `ArtifactSigning` PowerShell
module for the current user if it is missing.

Sign only installers you built from software you are entitled to redistribute, because your
name goes on them. Do not offer signing as a service to others.

An installer's window says who signed it, or "Not signed: publisher unknown". Signing is the only
thing that proves where one came from. The SHA-256 in its footer is checked before anything is read,
which catches a truncated copy or an edited manifest, but whoever edits a package can update that too.

### An installer in two files

A package past the 4 GiB Windows can start is built as `App_installer.exe`, a small loader, and
`App_installer.wamdata`, its data file. Sign the `.exe`, as usual. Do not sign anything else.

The loader's footer holds the data file's SHA-256 and length. A signature covers everything in the loader but
its own certificate table, so it covers that hash, and through it every byte of the data file:

- Changing one byte of the data file leaves the signature valid, because the signature cannot see it, and the
  installer refuses it before reading anything from it, because it no longer matches the footer.
- Changing the footer to match another data file makes the signature invalid. Windows reports `HashMismatch`.

`elevated_packaging.rs` shows both on a real runner, with a throwaway self-signed certificate and with no account
of this project's. What this does not do is make a signature mean more than it does. It says what is in the package
is what the signer built. It says nothing of what that package does.

Keep the two together. A signed loader without its data file installs nothing, and says what it is missing.

## The uninstaller

An install leaves a copy of the stub behind, for the app's entry in Settings to run. A validly signed
copy would be exactly the signed stub this page says must never exist, so it has no signature, even if
the installer it came from has one. When it is carved out of an installer, the package, or a loader's footer, and the certificate
table are cut off, and the table's location and the checksum, which signing writes into the headers,
are cleared. It comes out byte for byte the stub the installer was built from, and Windows reports it as
`NotSigned`. The elevated tests check that with `Get-AuthenticodeSignature`, for an installer and for a loader
given a valid signature from a throwaway certificate made on the runner.

So its UAC prompt says "Unknown publisher". That is the price of the rule, and what makes it safe is where
it is: a folder only administrators can write to.

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
  `AZURE_CLIENT_ID` and `AZURE_TENANT_ID` secrets, which identify but do not authenticate. Only the two
  jobs that sign use it.

Any step of a job that can ask GitHub for that token can sign with this certificate, so the release
workflow keeps everything that runs code from the repository or crates.io out of those jobs:

- **Building** happens in jobs with no secrets and a token that can only read. Build scripts, clippy,
  the tests, and `cargo-about`, pinned to one version, run there. They hand on the client, then the app,
  unsigned.
- **Signing** happens in jobs of their own, in the `release` environment, that run only pinned actions
  and no checkout. They take the file, sign in to Azure, sign it, and check the signature. The signing
  action does not restore its tools from GitHub's cache, which a build job could have written.
- **Publishing** happens in a job outside that environment, so Azure does not accept its token. It
  checks the app's signature again, records where the downloads were built, and publishes them. It is
  the only job that can write to the repository, and it runs nothing from it.

Cut a release by pushing a `v*` tag of three parts that matches the version in `Cargo.toml`, or the run stops at its
first check: see [Cutting a release](../CONTRIBUTING.md#cutting-a-release). To retry a run that failed before publishing, start the
workflow from that tag. If the release already exists, delete it first. Releases before `v0.5.0` were not signed, so this
path first runs at that tag.

Check the exe or the zip:

```powershell
gh attestation verify .\WinAppMigrator-v1.2.3.exe --repo RandyNorthrup/WinAppMigrator
(Get-AuthenticodeSignature .\WinAppMigrator-v1.2.3.exe).SignerCertificate.Subject
```

## Other protections for signed, elevated executables

A signed program that runs elevated is a target for loading someone else's code under its
name, so between them the app and the stub also, and the client as [above](#why-the-client-may-be-signed):

- resolve imported DLLs from System32 only (`/DEPENDENTLOADFLAG:0x800`), and DLLs loaded at
  runtime likewise (`SetDefaultDllDirectories`, first thing in `main`);
- link the C runtime statically, removing `VCRUNTIME140.dll` from the search entirely;
- run Windows tools by absolute path, with the Windows directory read from the admin-only
  registry rather than from an environment variable the user controls. winget is not a
  Windows tool. `PATH` only holds a shortcut to it in a folder the user can write to, so the
  installer asks Windows where its package is installed and checks that file's signature;
- take nothing about how a window is drawn from the environment, but `WGPU_BACKEND`, which can only turn Direct3D 12
  off. wgpu would otherwise go by
  `WGPU_DX12_AGILITY_SDK_PATH`, and have Direct3D run `D3D12Core.dll` from any folder the user names,
  past the restriction above. `check-real-windows.ps1` plants a stand-in to show that it does not;
- start every tool with a fixed list of environment variables, with its `Path` and `TEMP` from the Windows folder, and
  a runtime's setup program, DISM, or winget with a `TEMP` of its own only administrators can write to. PowerShell is a .NET program,
  and the user's environment could otherwise load a runtime or DLL into it, or replace the commands it runs, and its module path
  and tools come from the Windows folder too;
- ask `whoami` for elevation, which is not a .NET program, and treat not being able to tell as a
  failure rather than as not elevated;
- check that a prerequisite's installer is validly signed by Microsoft when it is downloaded
  and again just before running it;
- write the embedded stub straight into the installer, never through `%TEMP%`.

[cve]: https://msrc.microsoft.com/update-guide/vulnerability/CVE-2013-3900
[signing]: https://learn.microsoft.com/azure/artifact-signing/
