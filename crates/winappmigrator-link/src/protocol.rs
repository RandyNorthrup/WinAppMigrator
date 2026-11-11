//! What the builder asks a client, and what it answers, over a [`crate::channel::SecureChannel`].
//!
//! Each message is a JSON header, and for a read, the bytes read after it: a big-endian `u32` with
//! the header's length, the header, then the body.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{LinkError, Result};

/// The most one read asks for. A message holds this and its header.
pub const MAX_READ: u32 = 1024 * 1024;
/// The most entries one listing answers with, so that no answer is too long for a message. A folder
/// with more is listed in parts.
pub const MAX_LISTED: usize = 2000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum Request {
    /// What the client is and what it has.
    About,
    /// One path, described without following a link.
    Describe { path: String },
    /// A folder's entries, `MAX_LISTED` at a time from `from`, without following a link.
    List { path: String, from: usize },
    /// Part of a file.
    Read {
        path: String,
        offset: u64,
        length: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum Response {
    About(About),
    Described {
        entry: Entry,
    },
    Listed {
        entries: Vec<Entry>,
        more: bool,
    },
    /// The bytes read are the body. Fewer than were asked for means the file ended.
    Read,
    Failed {
        kind: FailureKind,
        message: String,
    },
}

/// What a client says of itself and the PC it is on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct About {
    pub client_version: String,
    pub computer_name: String,
    /// Such as `Windows 7 Professional Service Pack 1`.
    pub windows_name: String,
    /// `PROCESSOR_ARCHITECTURE` as that Windows records it: `x86`, `AMD64` or `ARM64`.
    pub architecture: String,
    /// When the snapshot everything is read from was taken, as that PC told the time.
    pub snapshot_taken: String,
}

/// A file, a folder, or a link, as the snapshot has it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub name: String,
    pub kind: EntryKind,
    pub len: u64,
    /// `FILE_ATTRIBUTE_*`.
    pub attributes: u32,
    /// `FILETIME`s: 100-nanosecond steps since 1601.
    pub modified: u64,
    pub created: u64,
    pub accessed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    File,
    Directory,
    /// A junction, a symbolic link or a mount point, which is never followed.
    Link,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    NotFound,
    Denied,
    /// The path is not one to ask about, or goes through a link.
    Refused,
    Other,
}

/// A header, and a body after it.
pub fn encode<T: Serialize>(header: &T, body: &[u8]) -> Result<Vec<u8>> {
    let header = serde_json::to_vec(header)
        .map_err(|error| LinkError::Protocol(format!("a message could not be written: {error}")))?;
    let length = u32::try_from(header.len())
        .map_err(|_| LinkError::Protocol("a message was too long".to_owned()))?;
    let mut message = Vec::with_capacity(4 + header.len() + body.len());
    message.extend_from_slice(&length.to_be_bytes());
    message.extend_from_slice(&header);
    message.extend_from_slice(body);
    Ok(message)
}

/// The header of a message, and its body.
pub fn decode<T: DeserializeOwned>(message: &[u8]) -> Result<(T, &[u8])> {
    let malformed = || LinkError::Protocol("a message was not one of this link's".to_owned());
    let length = message
        .get(..4)
        .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize)
        .ok_or_else(malformed)?;
    // Added with a check, as a length near the top of a 32-bit client's `usize` would wrap.
    let end = length.checked_add(4).ok_or_else(malformed)?;
    let header = message.get(4..end).ok_or_else(malformed)?;
    let body = message.get(end..).ok_or_else(malformed)?;
    let header = serde_json::from_slice(header).map_err(|_| malformed())?;
    Ok((header, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_header_and_its_body_come_back_as_they_went() {
        let request = Request::Read {
            path: r"Windows\x.dll".to_owned(),
            offset: 4096,
            length: 512,
        };
        let message = encode(&request, b"").ok().unwrap_or_default();
        let decoded = decode::<Request>(&message).ok();
        assert_eq!(
            decoded.map(|(header, body)| (header, body.len())),
            Some((request, 0))
        );

        let message = encode(&Response::Read, b"bytes").ok().unwrap_or_default();
        let decoded = decode::<Response>(&message).ok();
        assert_eq!(decoded, Some((Response::Read, &b"bytes"[..])));
    }

    #[test]
    fn what_is_not_a_message_is_refused() {
        for message in [
            &b""[..],
            &b"\0\0"[..],
            &b"\0\0\0\x09{}"[..],
            &b"\0\0\0\x02{]"[..],
        ] {
            assert!(decode::<Request>(message).is_err());
        }
    }
}
