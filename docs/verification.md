# Package And Restore Verification

`winappmigrator verify` validates package integrity and, when a target root is supplied, validates an applied restore.

## Package Verification

```powershell
cargo run -p winappmigrator-cli -- verify --package ".\AppMigrationPackage"
cargo run -p winappmigrator-cli -- verify --package ".\AppMigrationPackage.zip"
```

Checks include:

- manifest schema compatibility
- packaged file presence
- packaged file size
- packaged file SHA-256 hashes when captured

## Restore Verification

```powershell
cargo run -p winappmigrator-cli -- verify `
  --package ".\AppMigrationPackage.zip" `
  --target-root ".\RestoredApp"
```

Additional checks include:

- restored file presence, size, and SHA-256 hashes
- restored registry values with archived app paths rewritten to the target root
- restored user and machine environment variables
- service binary path validation
- scheduled task registration and command-path validation
- shortcut target validation

Driver and kernel components are intentionally marked as manual validation items. They must be reinstalled and certified through the vendor's signed driver package.

Reports are written as:

- `verification_report.json`
- `verification_report.md`
