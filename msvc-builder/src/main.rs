use crate::fixups::fixup_msvc_pe;
use crate::msvc::Msvc;
use crate::wine::{WinePrefix, WineTool};
use anyhow::{Context, Result};
use colored::Colorize;
use log::{debug, info, warn, LevelFilter};
use sha1::digest::FixedOutput;
use std::env;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

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
    let work_dir = tempdir::TempDir::new("msvc-work-dir")?;

    let wine_prefix = WinePrefix::new(WineTool::find()?, wine_prefix.as_ref().to_path_buf())?;

    let test_exes_path = PathBuf::from("./test_exes").canonicalize()?;
    let msvc_test_exes_path = test_exes_path.join("msvc");

    for file in std::fs::read_dir(&msvc_test_exes_path)
        .context("Reading directory with programs to be compiled")?
    {
        let file = file.context("Reading directory with programs to be compiled")?;
        let path = file.path();
        let file_type = file.file_type()?;

        if file_type.is_file() {
            if Some("cpp".as_ref()) == path.extension() || Some("c".as_ref()) == path.extension() {
                let output = path.with_extension("exe");

                info!(
                    "{} {:?} -> {:?}",
                    "BUILD".blue(),
                    path.strip_prefix(&msvc_test_exes_path).unwrap(),
                    output.strip_prefix(&msvc_test_exes_path).unwrap()
                );

                msvc.compile_exe(&wine_prefix, work_dir.path(), &[&path], &[], &output)
                    .context("Compiling exe")?;

                debug!("Zero out the timestamps...");

                fixup_msvc_pe(&output).context("Zeroing out the timestamps")?;
            }
        } else if file_type.is_dir() {
            warn!("Ignoring directory {:?}", path);
        } else {
            warn!("Ignoring unknown file type {:?} ({:?})", file_type, path);
        }
    }

    Ok(())
}
