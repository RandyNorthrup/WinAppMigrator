# Build WinAppMigrator Client for Windows 7 and later, as one 32-bit program that runs on 32-bit and
# 64-bit Windows 7, 8, 10 and 11. The app carries what this builds (see crates/winappmigrator-gui/build.rs),
# so run it before building the app for release.
#
# Rust's own targets for Windows stopped running on Windows 7 at 1.78. The `-win7-` targets keep it,
# and need the standard library built for them, which takes `-Zbuild-std`. RUSTC_BOOTSTRAP lets the
# pinned compiler do that, so the client is built by the same compiler as the rest. The C runtime is
# linked in, since Windows 7 has no Universal CRT of its own.
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)

rustup component add rust-src | Out-Null
$env:RUSTC_BOOTSTRAP = '1'
$env:RUSTFLAGS = '-C target-feature=+crt-static'
cargo build --release --locked -p winappmigrator-client `
    '-Zbuild-std=std,panic_abort' --target i686-win7-windows-msvc --target-dir target/win7
if ($LASTEXITCODE -ne 0) { throw "the client did not build: $LASTEXITCODE" }
$client = 'target\win7\i686-win7-windows-msvc\release\winappmigrator-client.exe'
"Built $client ($((Get-Item $client).Length) bytes)"
