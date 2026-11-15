//! Compresses each language's catalog into the build. Every installer carries every language, since the PC it runs on
//! may speak any of them, so they are kept small. A language's catalog is every `.ftl` file in its folder, in the order of
//! their names, so each part of the program keeps its words in a file of its own.
//!
//! WINAPPMIGRATOR_CATALOG_PARTS, when set, names the files to build in, by name without `.ftl`, separated by commas:
//! WinAppMigrator Client, which goes on old PCs and is built on its own by scripts/build-client.ps1, carries only its
//! own words and the shared ones. Not a Cargo feature, since a feature is turned on for every crate in a build that
//! includes the client, and would leave the app without its words.

use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::DeflateEncoder;

const CATALOG_EXTENSION: &str = "ftl";
const PARTS_VARIABLE: &str = "WINAPPMIGRATOR_CATALOG_PARTS";

fn main() -> Result<(), Box<dyn Error>> {
    let locales = Path::new("locales");
    println!("cargo:rerun-if-changed=locales");
    println!("cargo:rerun-if-env-changed={PARTS_VARIABLE}");
    let parts: Option<Vec<String>> = std::env::var(PARTS_VARIABLE).ok().map(|value| {
        value
            .split(',')
            .map(|part| part.trim().to_owned())
            .filter(|part| !part.is_empty())
            .collect()
    });
    let out = PathBuf::from(std::env::var_os("OUT_DIR").ok_or("OUT_DIR is not set")?);

    let mut catalogs: Vec<(String, Vec<PathBuf>)> = Vec::new();
    for entry in fs::read_dir(locales)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let mut files: Vec<PathBuf> = fs::read_dir(entry.path())?
            .filter_map(|file| file.ok().map(|file| file.path()))
            .filter(|file| file.extension().is_some_and(|ext| ext == CATALOG_EXTENSION))
            .filter(|file| {
                parts.as_ref().is_none_or(|parts| {
                    file.file_stem().is_some_and(|stem| {
                        parts
                            .iter()
                            .any(|part| stem.eq_ignore_ascii_case(part.as_str()))
                    })
                })
            })
            .collect();
        if files.is_empty() {
            continue;
        }
        files.sort();
        catalogs.push((entry.file_name().to_string_lossy().into_owned(), files));
    }
    catalogs.sort();

    let mut table = String::from(
        "/// Each shipped language's tag and its catalog, compressed. Written by build.rs.\n\
         pub(crate) static CATALOGS: &[(&str, &[u8])] = &[\n",
    );
    for (tag, files) in &catalogs {
        let mut source = Vec::new();
        for file in files {
            // Comments are for translators, not for the program. A line that starts with `#` is always one: a
            // message's own lines after its first are indented.
            for line in fs::read(file)?.split(|byte| *byte == b'\n') {
                if line.first() != Some(&b'#') {
                    source.extend(line.strip_suffix(b"\r").unwrap_or(line));
                    source.push(b'\n');
                }
            }
        }
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&source)?;
        let file = format!("{tag}.ftl.deflate");
        fs::write(out.join(&file), encoder.finish()?)?;
        writeln!(
            table,
            "    ({tag:?}, include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{file}\"))),"
        )?;
    }
    table.push_str("];\n");
    fs::write(out.join("catalogs.rs"), table)?;
    Ok(())
}
