//! Compresses each language's catalog into the build. Every installer carries every language, since the PC it runs on
//! may speak any of them, so they are kept small. A language's catalog is every `.ftl` file in its folder, in the order of
//! their names, so each part of the program keeps its words in a file of its own.

use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::DeflateEncoder;

const CATALOG_EXTENSION: &str = "ftl";

fn main() -> Result<(), Box<dyn Error>> {
    let locales = Path::new("locales");
    println!("cargo:rerun-if-changed=locales");
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
            source.extend(fs::read(file)?);
            // A file need not end in a line break, and the next must start on a line of its own.
            source.push(b'\n');
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
