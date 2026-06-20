# Inventory Snapshots

Inventory snapshots are read-only records of installed application metadata from per-user and per-machine uninstall registry locations.

CLI:

```powershell
cargo run -p winappmigrator-cli -- snapshot --out ".\inventory-snapshot"
```

GUI:

- Choose an output directory.
- Use `Save inventory snapshot`.

Artifacts:

- `inventory_snapshot.json`
- `inventory_snapshot.md`

Snapshots do not package files, export secrets, or modify the system. They are intended for pre-migration inventory review and change comparison between operator runs.
