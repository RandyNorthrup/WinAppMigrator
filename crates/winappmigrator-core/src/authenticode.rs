//! Reading Authenticode signatures, through PowerShell's `Get-AuthenticodeSignature`.
//!
//! A file's hash only says it has not changed since someone recorded the hash, and the
//! manifest holding that hash is not authenticated. A signature says who made the file.

use std::path::Path;

use serde::Deserialize;

use crate::path_utils::{
    POWERSHELL_IMPORT_SECURITY, POWERSHELL_IMPORT_UTILITY, POWERSHELL_UTF8_OUTPUT,
    powershell_command,
};

/// The path travels in this variable, so that no quoting can go wrong. The script only
/// reads it through the provider, as a literal path.
const PATH_VARIABLE: &str = "WAM_SIGNATURE_PATH";
const SIGNATURE_SCRIPT: &str = "$s = Get-AuthenticodeSignature -LiteralPath $env:WAM_SIGNATURE_PATH; \
     [pscustomobject]@{ \
       Status = [string]$s.Status; \
       Subject = $s.SignerCertificate.Subject; \
       Issuer = $s.SignerCertificate.Issuer \
     } | ConvertTo-Json -Compress";
const STATUS_VALID: &str = "Valid";
const MICROSOFT_ORGANIZATION: &str = "Microsoft Corporation";

/// What Windows says about a file's signature.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Signature {
    pub status: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub issuer: Option<String>,
}

impl Signature {
    /// Windows checked the signature, the file matches it, and the signer is trusted.
    pub fn is_valid(&self) -> bool {
        self.status == STATUS_VALID
    }

    /// Valid, and both the signer and whoever issued its certificate are Microsoft. A
    /// certificate authority will not put `Microsoft Corporation` in someone else's
    /// certificate, and checking the issuer as well means it would have had to be one of
    /// Microsoft's own. The text is compared whole, so `Not Microsoft Corporation` fails.
    pub fn is_microsoft(&self) -> bool {
        let is_microsoft = |name: &Option<String>| {
            name.as_deref()
                .and_then(|name| name_attribute(name, "O"))
                .as_deref()
                == Some(MICROSOFT_ORGANIZATION)
        };
        self.is_valid() && is_microsoft(&self.subject) && is_microsoft(&self.issuer)
    }

    /// The signer's name as a person would say it, if it has one.
    pub fn signer_name(&self) -> Option<String> {
        let subject = self.subject.as_deref()?;
        name_attribute(subject, "CN").or_else(|| name_attribute(subject, "O"))
    }
}

/// Read a file's signature. An error means the question could not be asked, which is
/// never to be taken as a good answer.
pub fn read_signature(path: &Path) -> Result<Signature, String> {
    let output = powershell_command(format!(
                "{POWERSHELL_UTF8_OUTPUT}{POWERSHELL_IMPORT_SECURITY}{POWERSHELL_IMPORT_UTILITY}{SIGNATURE_SCRIPT}"
            ))
        .env(PATH_VARIABLE, path)
        .output()
        .map_err(|error| format!("could not start PowerShell: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(text.trim_start_matches('\u{feff}').trim())
        .map_err(|error| format!("could not read the signature: {error}"))
}

/// Check that a file is validly signed by Microsoft, with the reason if it is not.
pub fn check_microsoft_signature(path: &Path) -> Result<(), String> {
    let signature = read_signature(path)?;
    if signature.is_microsoft() {
        return Ok(());
    }
    if signature.is_valid() {
        return Err(format!(
            "it is signed by {}, not Microsoft",
            signature
                .signer_name()
                .unwrap_or_else(|| "someone else".to_owned())
        ));
    }
    Err(format!("its signature status is {}", signature.status))
}

/// One attribute from a certificate name such as
/// `CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US`.
/// A value holding a comma is quoted, and a quoted comma or `O=` is part of that value.
fn name_attribute(name: &str, key: &str) -> Option<String> {
    let mut quoted = false;
    let mut start = 0;
    for (index, character) in name.char_indices().chain([(name.len(), ',')]) {
        match character {
            '"' => quoted = !quoted,
            ',' if !quoted => {
                let part = name.get(start..index)?.trim();
                start = index + character.len_utf8();
                if let Some((attribute, value)) = part.split_once('=')
                    && attribute.trim().eq_ignore_ascii_case(key)
                {
                    let value = value.trim();
                    let value = value
                        .strip_prefix('"')
                        .and_then(|value| value.strip_suffix('"'))
                        .unwrap_or(value);
                    return Some(value.replace("\"\"", "\""));
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path_utils::system_root;

    const MICROSOFT_SUBJECT: &str =
        "CN=Microsoft Corporation, O=Microsoft Corporation, L=Redmond, S=Washington, C=US";
    const MICROSOFT_ISSUER: &str =
        "CN=Microsoft Code Signing PCA 2024, O=Microsoft Corporation, C=US";

    fn signature(status: &str, subject: &str, issuer: &str) -> Signature {
        Signature {
            status: status.to_owned(),
            subject: Some(subject.to_owned()),
            issuer: Some(issuer.to_owned()),
        }
    }

    #[test]
    fn the_real_microsoft_names_pass() {
        assert!(signature("Valid", MICROSOFT_SUBJECT, MICROSOFT_ISSUER).is_microsoft());
        assert!(
            signature(
                "Valid",
                "CN=Microsoft Windows, O=Microsoft Corporation, L=Redmond, S=Washington, C=US",
                "CN=Microsoft Windows Production PCA 2011, O=Microsoft Corporation, L=Redmond, \
                 S=Washington, C=US"
            )
            .is_microsoft()
        );
    }

    #[test]
    fn any_status_but_valid_fails() {
        for status in [
            "NotSigned",
            "HashMismatch",
            "NotTrusted",
            "UnknownError",
            "valid ",
        ] {
            assert!(
                !signature(status, MICROSOFT_SUBJECT, MICROSOFT_ISSUER).is_microsoft(),
                "{status}"
            );
        }
    }

    #[test]
    fn a_name_merely_mentioning_microsoft_fails() {
        for subject in [
            "CN=Microsoft Corporation",
            "CN=Fabrikam, O=Not Microsoft Corporation, C=US",
            "CN=Fabrikam, O=Microsoft Corporation Ltd, C=US",
            r#"CN="Fabrikam, O=Microsoft Corporation", O=Fabrikam, C=US"#,
            "OU=Microsoft Corporation, O=Fabrikam",
        ] {
            assert!(
                !signature("Valid", subject, MICROSOFT_ISSUER).is_microsoft(),
                "{subject}"
            );
        }
        assert!(
            !signature("Valid", MICROSOFT_SUBJECT, "CN=Other CA, O=Other, C=US").is_microsoft(),
            "the issuer has to be Microsoft too"
        );
    }

    #[test]
    fn a_missing_certificate_fails() {
        let unsigned = Signature {
            status: "NotSigned".to_owned(),
            subject: None,
            issuer: None,
        };
        assert!(!unsigned.is_valid());
        assert!(!unsigned.is_microsoft());
        assert_eq!(unsigned.signer_name(), None);

        let valid_without_names = Signature {
            status: "Valid".to_owned(),
            subject: None,
            issuer: None,
        };
        assert!(!valid_without_names.is_microsoft());
    }

    #[test]
    fn the_signers_name_is_the_common_name_and_can_hold_a_comma() {
        assert_eq!(
            signature("Valid", MICROSOFT_SUBJECT, "")
                .signer_name()
                .as_deref(),
            Some("Microsoft Corporation")
        );
        assert_eq!(
            signature(
                "Valid",
                r#"CN="Fabrikam, Inc.", O="Fabrikam, Inc.", C=US"#,
                ""
            )
            .signer_name()
            .as_deref(),
            Some("Fabrikam, Inc.")
        );
        assert_eq!(
            signature("Valid", "O=Fabrikam, C=US", "")
                .signer_name()
                .as_deref(),
            Some("Fabrikam")
        );
    }

    #[test]
    fn a_windows_tool_passes_the_real_check() {
        let tool = system_root().join("System32").join("sc.exe");
        assert_eq!(check_microsoft_signature(&tool), Ok(()));
    }

    #[test]
    fn an_unsigned_file_fails_the_real_check() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        let file = folder.path().join("it's a test ‘quoted’ & odd.exe");
        std::fs::write(&file, b"MZ not really a program")?;
        let refused = check_microsoft_signature(&file);
        assert!(
            refused
                .as_ref()
                .is_err_and(|reason| reason.contains("status")),
            "{refused:?}"
        );
        Ok(())
    }

    #[test]
    fn a_changed_copy_of_a_signed_file_fails_the_real_check() -> std::io::Result<()> {
        let folder = tempfile::tempdir()?;
        let copy = folder.path().join("sc.exe");
        let mut bytes = std::fs::read(system_root().join("System32").join("sc.exe"))?;
        if let Some(last) = bytes.last_mut() {
            *last ^= 0xFF;
        }
        std::fs::write(&copy, bytes)?;
        assert!(check_microsoft_signature(&copy).is_err());
        Ok(())
    }

    #[test]
    fn a_file_that_is_not_there_fails_the_real_check() {
        assert!(check_microsoft_signature(Path::new(r"Z:\wam-no-such-file.exe")).is_err());
    }
}
