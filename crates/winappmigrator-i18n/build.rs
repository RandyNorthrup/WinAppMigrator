//! Compresses each language's catalog into the build. Every installer carries every language, since the PC it runs on
//! may speak any of them, so they are kept small.

use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::DeflateEncoder;

const CATALOG_FILE: &str = "winappmigrator.ftl";

fn main() -> Result<(), Box<dyn Error>> {
    let locales = Path::new("locales");
    println!("cargo:rerun-if-changed=locales");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").ok_or("OUT_DIR is not set")?);

    let mut tags: Vec<String> = Vec::new();
    for entry in fs::read_dir(locales)? {
        let entry = entry?;
        let catalog = entry.path().join(CATALOG_FILE);
        if entry.file_type()?.is_dir() && catalog.is_file() {
            tags.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    tags.sort();

    let mut table = String::from(
        "/// Each shipped language's tag and its catalog, compressed. Written by build.rs.\n\
         pub(crate) static CATALOGS: &[(&str, &[u8])] = &[\n",
    );
    for tag in &tags {
        let source = fs::read(locales.join(tag).join(CATALOG_FILE))?;
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
