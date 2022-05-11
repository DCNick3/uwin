use crate::fixups::{fixup_file, patch_msvc_import_lib};
use crate::{fixup_msvc_pe, Msvc, WinePrefix};
use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::{info, warn};
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum Kind {
    Exe,
    Dll,
}
impl Default for Kind {
    fn default() -> Self {
        Self::Exe
    }
}

#[derive(Deserialize, Debug)]
pub struct FileBuildParams {
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub link_deps: Vec<String>,
}

lazy_static! {
    static ref FILE_PARAMS_REGEX: Regex =
        Regex::new(r"(?ms:/\*UW_BUILD:(.*):UW_BUILD\*/)").unwrap();
}

fn parse_magic_comment(path: &Path) -> Result<FileBuildParams> {
    let contents = std::fs::read_to_string(path).context("Reading the source file")?;

    let magic_comment = FILE_PARAMS_REGEX
        .captures(&contents)
        .context("Could not find magic comment with file params")?;
    let file_params = magic_comment.get(1).unwrap().as_str();
    let file_params: FileBuildParams =
        serde_json::from_str(file_params).context("Could not parse magic comment")?;

    Ok(file_params)
}

pub struct CompileUnit {
    name: String,
    kind: Kind,
    sources: Vec<PathBuf>,
    expected_outputs: Vec<PathBuf>,
    link_dependencies: Vec<String>,
}

impl CompileUnit {
    // TODO: support multiple source files?
    pub fn parse(_base_dir: &Path, path: &Path) -> Result<Self> {
        let build_params = parse_magic_comment(path).context("Parsing magic comment")?;

        let name = path.file_stem().unwrap().to_str().unwrap().to_string();

        info!("COLLECT {}: {:?}", name, build_params);

        let primary_output = match build_params.kind {
            Kind::Exe => path.with_extension("exe"),
            Kind::Dll => path.with_extension("dll"),
        };

        let outputs = match build_params.kind {
            Kind::Exe => vec![primary_output],
            Kind::Dll => vec![primary_output, path.with_extension("lib")],
        };

        Ok(Self {
            name,
            kind: build_params.kind,
            sources: vec![path.to_path_buf()],
            expected_outputs: outputs,
            link_dependencies: build_params.link_deps,
        })
    }

    fn link(&self) -> Result<&Path> {
        if self.kind != Kind::Dll {
            return Err(anyhow!(
                "Attempt to link to something that is not a dll (dep with name {})",
                self.name
            ));
        }

        Ok(self.expected_outputs.last().unwrap()) // !!! kinda unsafe, but whatever. We rely on the fact that the .lib is stored as the last item
    }

    pub fn build(
        &self,
        msvc: &Msvc,
        wine_prefix: &WinePrefix,
        work_dir: &Path,
        store: &BTreeMap<String, CompileUnit>,
    ) -> Result<()> {
        info!("BUILD   {}", self.name);

        let mut link_files = Vec::new();
        // check that link deps are DLLs
        for dep in &self.link_dependencies {
            let dep = store.get(dep).with_context(|| {
                format!("Could not find dependency with name {} in the store", dep)
            })?;

            link_files.push(
                dep.link()
                    .with_context(|| format!("Resolving linking dep of {}", self.name))?,
            );
        }

        for output in &self.expected_outputs {
            if std::fs::remove_file(output).is_err() {
                warn!("Could not remove file {:?}", output);
            }
        }

        let primary_output = self.expected_outputs.first().unwrap();

        match self.kind {
            Kind::Exe => msvc
                .compile_exe(
                    wine_prefix,
                    work_dir,
                    &self.sources.iter().map(|s| s.as_path()).collect::<Vec<_>>(),
                    &[],
                    &link_files,
                    primary_output,
                )
                .context("Compiling exe")?,
            Kind::Dll => msvc
                .compile_dll(
                    wine_prefix,
                    work_dir,
                    &self.sources.iter().map(|s| s.as_path()).collect::<Vec<_>>(),
                    &[],
                    &link_files,
                    primary_output,
                )
                .context("Compiling dll")?,
        }

        fixup_file(primary_output, fixup_msvc_pe).context("Zeroing out the timestamps")?;

        if self.kind == Kind::Dll {
            let path = &self.expected_outputs[1];

            let exp_file = path.with_extension("exp");
            std::fs::remove_file(exp_file)?;

            let mut imp_file = OpenOptions::new().write(true).read(true).open(path)?;

            patch_msvc_import_lib(&mut imp_file).expect("Opening import lib");
        }

        Ok(())
    }

    pub fn dependencies(&self) -> impl Iterator<Item = &str> {
        self.link_dependencies.iter().map(|s| s.as_str())
    }
}

pub fn build(
    msvc: &Msvc,
    wine_prefix: &WinePrefix,
    work_dir: &Path,
    base_dir: &Path,
    input_files: &[&Path],
) -> Result<()> {
    let mut store = BTreeMap::new();

    for item in input_files {
        let item =
            CompileUnit::parse(base_dir, item).with_context(|| format!("Parsing {:?}", item))?;

        let old = store.insert(item.name.clone(), item);
        if let Some(old) = old {
            return Err(anyhow!("Found two compile units with name {}", old.name));
        }
    }

    let mut building = HashSet::new();
    let mut built = HashSet::new();

    fn process<'a>(
        building: &mut HashSet<&'a str>,
        built: &mut HashSet<&'a str>,
        store: &'a BTreeMap<String, CompileUnit>,
        msvc: &Msvc,
        wine_prefix: &WinePrefix,
        work_dir: &Path,
        unit: &'a CompileUnit,
    ) -> Result<()> {
        if built.contains(unit.name.as_str()) {
            return Ok(());
        }

        building.insert(unit.name.as_str());
        for dep in unit.dependencies() {
            if building.contains(dep) && !built.contains(dep) {
                return Err(anyhow!("Found a dependency loop (on item = {})", unit.name));
            }

            let dep = store
                .get(dep)
                .with_context(|| format!("Could not find a dependency {}", dep))?;

            process(building, built, store, msvc, wine_prefix, work_dir, dep)?;
        }

        unit.build(msvc, wine_prefix, work_dir, store)?;

        built.insert(unit.name.as_str());

        Ok(())
    }

    for (_, unit) in store.iter() {
        process(
            &mut building,
            &mut built,
            &store,
            msvc,
            wine_prefix,
            work_dir,
            unit,
        )
        .with_context(|| format!("Building {}", unit.name))?;
    }

    Ok(())
}
