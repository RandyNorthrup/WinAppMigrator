# Code Signing

Signing is intentionally disabled for the current development milestone.

Future release builds should add Authenticode signing for:

- `winappmigrator.exe`
- `winappmigrator-gui.exe`
- self-extracting package executables produced by `winappmigrator package --sfx`

Required future inputs:

- EV or OV code-signing certificate source
- Timestamp server URL
- Certificate storage policy
- CI secret names and rotation policy
- Release approval workflow

No private keys, certificate passwords, or signing secrets may be committed to this repository.
