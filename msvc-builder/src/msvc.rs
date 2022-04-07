use crate::{FixedOutput, WinePrefix};
use anyhow::{Context, Result};
use checked_command::CommandExt;
use diff::Diff;
use log::debug;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MSVC_FILES: &str = include_str!("VC98.txt");

fn check_msvc(path: &Path) -> Result<()> {
    debug!("Checking MSVC distribution in {:?}", path);

    let required_files = MSVC_FILES
        .lines()
        .map(|line| {
            let vals = line.splitn(2, "  ").collect::<Vec<_>>();
            assert_eq!(vals.len(), 2);
            let hash = vals[0].to_string();
            let filename = vals[1].to_string();
            (filename, hash)
        })
        .collect::<BTreeMap<_, _>>();

    let have_files = WalkDir::new(path)
        .into_iter()
        .map(|filename| -> anyhow::Result<_> {
            let filename = filename?;
            if !filename.file_type().is_file() {
                return Ok(None);
            }

            let filename = filename.path();
            let mut file = std::fs::File::open(filename)?;
            let mut sha1 = sha1::Sha1::default();

            std::io::copy(&mut file, &mut sha1)?;

            let sha1 = base16ct::lower::encode_string(&sha1.finalize_fixed());

            Ok(Some((
                filename.strip_prefix(path)?.to_str().unwrap().to_string(),
                sha1,
            )))
        })
        .filter_map(|f| match f {
            Ok(Some(f)) => Some(Ok(f)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        })
        .collect::<anyhow::Result<BTreeMap<_, _>>>()?;

    let diff = required_files.diff(&have_files);

    if !diff.altered.is_empty() || !diff.removed.is_empty() {
        let mut report = String::new();

        for removed in diff.removed.iter() {
            writeln!(report, "{:>20}: {}", "Missing file", removed)?;
        }

        for (filename, have_hash) in diff.altered.iter() {
            if let Some(expected_hash) = required_files.get(filename) {
                writeln!(
                    report,
                    "{:>20}  {}: expected {}, but actually is {}",
                    "Hash mismatch for",
                    filename,
                    expected_hash,
                    have_hash.as_ref().unwrap()
                )?;
            } else {
                writeln!(report, "{:>20}: {}", "Extra file", filename)?;
            }
        }

        return Err(anyhow::Error::msg(format!(
            "MSVC distribution does match up with what we expect:\n{}",
            report
        )));
    }

    Ok(())
}

pub struct Msvc {
    // path: PathBuf,
    cl_path: PathBuf,
    // link_path: PathBuf,
    includes_path: PathBuf,
    libs_path: PathBuf,
}

impl Msvc {
    pub fn new(path: PathBuf) -> Result<Self> {
        let path = path
            .canonicalize()
            .context("Canonicalizing path to MSVC distribution")?;

        let cl_path = path.join("bin/cl.exe");
        let link_path = path.join("bin/link.exe");
        let includes_path = path.join("include");
        let libs_path = path.join("lib");

        check_msvc(&path).context("Checking MSVC distribution")?;

        assert!(cl_path.is_file());
        assert!(link_path.is_file());
        assert!(includes_path.is_dir());
        assert!(libs_path.is_dir());

        Ok(Self {
            // path,
            cl_path,
            // link_path,
            includes_path,
            libs_path,
        })
    }

    fn env(&self, wine: &WinePrefix) -> Result<BTreeMap<String, String>> {
        Ok(BTreeMap::from([
            (
                "INCLUDE".to_string(),
                wine.windows_path(&self.includes_path)?.to_string(),
            ),
            (
                "LIB".to_string(),
                wine.windows_path(&self.libs_path)?.to_string(),
            ),
        ]))
    }

    // TODO: provide a better API for compilation options
    pub fn compile_exe(
        &self,
        wine: &WinePrefix,
        work_dir: &Path,
        source_files: &[&Path],
        flags: &[&OsStr],
        output_path: &Path,
    ) -> Result<()> {
        let cl = wine
            .windows_path(&self.cl_path)
            .map(|f| f.to_string())
            .context("Getting windows path to cl")?;
        let source_files = source_files
            .iter()
            .map(|f| wine.windows_path(f).map(|f| f.to_string()))
            .collect::<Result<Vec<_>>>()
            .context("Getting windows path to cpp files")?;
        let output_path = wine
            .windows_path(output_path)
            .map(|f| f.to_string())
            .context("Getting windows path to output")?;

        let mut command = wine.command_inside(cl)?;
        command
            .envs(self.env(wine)?)
            .current_dir(work_dir)
            .args(flags)
            .args(&source_files)
            .arg(format!("/Fe{}", output_path))
            .args([
                "/link",
                "kernel32.lib",
                "user32.lib",
                "gdi32.lib",
                "winspool.lib",
                "comdlg32.lib",
                "advapi32.lib",
                "shell32.lib",
                "ole32.lib",
                "oleaut32.lib",
                "uuid.lib",
                "odbc32.lib",
                "odbccp32.lib",
                "/nologo",
                "/subsystem:windows",
                "/machine:I386",
                "/debug",
                "/debugtype:coff",
            ]);

        debug!("Going to run MSVC as {:?}", command);

        command.checked_status().context("Running MSVC")?;

        Ok(())
    }
}
