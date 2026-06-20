# Code Signing

Authenticode signing is wired for manual GitHub Actions release workflows when signing secrets are provided. Local development and pull-request CI remain unsigned.

Release builds should sign:

- `winappmigrator.exe`
- `winappmigrator-gui.exe`
- self-extracting package executables produced by `winappmigrator package --sfx`

Selected policy:

- Certificate: organization-controlled OV or EV Authenticode certificate exported to a password-protected PFX only for CI use.
- Timestamp server: `http://timestamp.digicert.com` by default, overridable with `WINAPPMIGRATOR_TIMESTAMP_URL`.
- CI secrets:
  - `WINAPPMIGRATOR_SIGNING_CERT_BASE64`: base64-encoded PFX.
  - `WINAPPMIGRATOR_SIGNING_CERT_PASSWORD`: PFX password.
- CI activation: manual `workflow_dispatch` only, and only when both signing secrets are present.
- Local signing helper: `scripts\sign-release.ps1`.
- Rotation: replace the PFX secret before certificate expiration or immediately after suspected exposure.

No private keys, certificate passwords, or signing secrets may be committed to this repository.
