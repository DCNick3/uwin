use crate::fixups::fixup_msvc_pe;
use crate::msvc::Msvc;
use crate::wine::{WinePrefix, WineTool};
use anyhow::{Context, Result};
use log::{info, warn, LevelFilter};
use sha1::digest::FixedOutput;
use std::env;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

mod comp_dag;
mod fixups;
mod msvc;
mod wine;

enum WinePrefixPath {
    PathBuf(PathBuf),
    TempDir(TempDir),
}

impl AsRef<Path> for WinePrefixPath {
    fn as_ref(&self) -> &Path {
        match self {
            WinePrefixPath::PathBuf(path) => &path,
            WinePrefixPath::TempDir(temp) => temp.path(),
        }
    }
}

fn get_wine_prefix() -> Result<WinePrefixPath> {
    if let Some(path) = env::var_os("WINEPREFIX") {
        let path = PathBuf::from(path);
        info!("Using {:?} as wine prefix", path);
        Ok(WinePrefixPath::PathBuf(path))
    } else {
        info!("Using temp directory as a wine prefix");
        Ok(WinePrefixPath::TempDir(tempdir::TempDir::new(
            "msvc-wine-prefix",
        )?))
    }
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let msvc_path = PathBuf::from(env::var_os("MSVC_PATH").context("Cannot get MSVC_PATH env")?);
    let wine_prefix = get_wine_prefix()?;

    let msvc = Msvc::new(msvc_path)?;

    // MSVC likes to create some BS files in the working dir
    // don't let it do it
    let work_dir = TempDir::new("msvc-work-dir")?;

    let wine_prefix = WinePrefix::new(WineTool::find()?, wine_prefix.as_ref().to_path_buf())?;

    let test_exes_path = PathBuf::from("./test_exes").canonicalize()?;
    let msvc_test_exes_path = test_exes_path.join("msvc");

    let mut files = Vec::new();

    for file in std::fs::read_dir(&msvc_test_exes_path)
        .context("Reading directory with programs to be compiled")?
    {
        let file = file.context("Reading directory with programs to be compiled")?;

        let path = file.path();
        let file_type = file.file_type().unwrap();

        if file_type.is_file() {
            if Some("cpp".as_ref()) == path.extension() || Some("c".as_ref()) == path.extension() {
                files.push(path)
            }
        } else if file_type.is_dir() {
            warn!("Ignoring directory {:?}", path);
        } else {
            warn!("Ignoring unknown file type {:?} ({:?})", file_type, path);
        }
    }

    comp_dag::build(
        &msvc,
        &wine_prefix,
        work_dir.path(),
        &msvc_test_exes_path,
        &files.iter().map(|f| f.as_path()).collect::<Vec<_>>(),
    )?;

    Ok(())
}
