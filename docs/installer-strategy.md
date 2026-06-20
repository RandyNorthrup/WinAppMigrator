# Installer Strategy

Current decision: keep portable `.zip` as the release packaging format.

MSI, MSIX, and setup-executable installers are not selected for the current release line because WinAppMigrator is itself a migration and packaging tool, and portable distribution keeps the restore posture explicit and auditable.

Reconsider installer technology when one of these becomes true:

- enterprise deployment requires fleet-wide installation through software distribution tooling
- code signing is active and release promotion needs installed Start Menu entries
- update/uninstall lifecycle management becomes a product requirement
- shell integration or protocol registration is added

If installer packaging is later selected, prefer MSI for broad enterprise deployment compatibility unless MSIX isolation becomes a hard requirement.
