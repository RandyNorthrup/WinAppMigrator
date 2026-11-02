use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use sha2::{Digest, Sha256};

use crate::errors::{Result, WinAppMigratorError, panic_text};
use crate::packer::worker_count;

const HASH_BUFFER_BYTES: usize = 1024 * 1024;
/// Reading files back is mostly waiting for the disk, and past this it mostly waits for itself.
const MOST_READ_BACK_THREADS: usize = 8;

pub fn sha256_file(path: &Path) -> Result<String> {
    let file = File::open(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let mut reader = BufReader::with_capacity(HASH_BUFFER_BYTES, file);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];

    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|source| WinAppMigratorError::io(path, source))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(hex_lower(&hasher.finalize()))
}

/// What came of reading a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReadBack {
    pub size: u64,
    pub sha256: String,
}

fn read_back_file(path: &Path, is_cancelled: &(dyn Fn() -> bool + Sync)) -> Result<ReadBack> {
    let mut file = File::open(path).map_err(|source| WinAppMigratorError::io(path, source))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; HASH_BUFFER_BYTES];
    let mut size = 0_u64;
    loop {
        if is_cancelled() {
            return Err(WinAppMigratorError::Cancelled);
        }
        let read = file
            .read(&mut buffer)
            .map_err(|source| WinAppMigratorError::io(path, source))?;
        if read == 0 {
            return Ok(ReadBack {
                size,
                sha256: hex_lower(&hasher.finalize()),
            });
        }
        hasher.update(&buffer[..read]);
        size += read as u64;
    }
}

/// Read each of these files from the disk and hash it, on as many threads as make sense, each with
/// a buffer of its own. A file that cannot be read says so in its own result, and a cancel stops all
/// of it.
pub(crate) fn read_back_files(
    paths: &[PathBuf],
    is_cancelled: &(dyn Fn() -> bool + Sync),
) -> Result<HashMap<PathBuf, Result<ReadBack>>> {
    let next = AtomicUsize::new(0);
    let results = Mutex::new(HashMap::with_capacity(paths.len()));
    let threads = worker_count()
        .min(MOST_READ_BACK_THREADS)
        .min(paths.len().max(1));
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                while !is_cancelled() {
                    let Some(path) = paths.get(next.fetch_add(1, Ordering::Relaxed)) else {
                        return;
                    };
                    // A panic reading a file goes no further than this, and counts as not read.
                    let result =
                        catch_unwind(AssertUnwindSafe(|| read_back_file(path, is_cancelled)))
                            .unwrap_or_else(|panic| {
                                Err(WinAppMigratorError::InvalidInput(format!(
                                    "{} could not be read back: {}",
                                    path.display(),
                                    panic_text(panic.as_ref())
                                )))
                            });
                    if let Ok(mut results) = results.lock() {
                        results.insert(path.clone(), result);
                    }
                }
            });
        }
    });
    if is_cancelled() {
        return Err(WinAppMigratorError::Cancelled);
    }
    Ok(results
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner))
}

/// Encode bytes as a lowercase hex string (e.g. a SHA-256 digest or a raw
/// registry value). Allocation-free per byte, unlike a `format!("{:02x}")` loop.
pub fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_files_back_gives_each_one_what_is_really_in_it() -> Result<()> {
        let work = tempfile::tempdir().map_err(|error| WinAppMigratorError::io("temp", error))?;
        let mut paths = Vec::new();
        for index in 0..40 {
            let path = work.path().join(format!("{index}.txt"));
            std::fs::write(&path, vec![index as u8; index * 1000])
                .map_err(|error| WinAppMigratorError::io(&path, error))?;
            paths.push(path);
        }
        let missing = work.path().join("not there");
        paths.push(missing.clone());
        let mut read = read_back_files(&paths, &|| false)?;
        assert!(read.remove(&missing).is_some_and(|result| result.is_err()));
        assert_eq!(read.len(), 40);
        for (path, result) in read {
            let back = result?;
            assert_eq!(back.sha256, sha256_file(&path)?);
            assert_eq!(
                back.size,
                std::fs::metadata(&path)
                    .map_err(|error| WinAppMigratorError::io(&path, error))?
                    .len()
            );
        }
        Ok(())
    }

    #[test]
    fn cancelling_stops_reading_files_back() {
        let found = read_back_files(&[PathBuf::from("anything")], &|| true);
        assert!(matches!(found, Err(WinAppMigratorError::Cancelled)));
    }

    #[test]
    fn formats_hex_lowercase() {
        assert_eq!(hex_lower(&[0x00, 0xab, 0xff]), "00abff");
    }
}
