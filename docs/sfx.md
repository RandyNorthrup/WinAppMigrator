# Self-Extracting Package UX

Current decision: keep SFX packages as a minimal standalone restore wrapper for this release line.

Rationale:

- Restore must remain dry-run by default.
- Operators already have a richer review surface through the GUI and CLI reports.
- A wizard-style installer would duplicate installer responsibilities without replacing the need for package verification, admin elevation checks, and explicit apply mode.

The SFX wrapper should continue to:

- print usage when launched without `--target-root`
- run dry-run restore unless `--apply` is supplied
- support `--force` for explicit overwrite
- use the same shared restore and verification core paths as the CLI

Future installer-style UX should be reconsidered only if portable `.zip` plus the GUI is not enough for enterprise distribution.
