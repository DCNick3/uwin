use anyhow::{Context, Result};
use checked_command::CommandExt;
use command_group::{CommandGroup, GroupChild};
use log::debug;
use path_absolutize::Absolutize;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use winepath::{WineConfig, WinePath};

#[derive(Debug)]
pub struct WineTool {
    wine: PathBuf,
    wine_server: PathBuf,
}

impl WineTool {
    pub fn find() -> Result<Self> {
        let wine = which::which("wine").context("Searching for wine")?;
        let wine_server = which::which("wineserver").context("Searching for wineserver")?;
        Ok(Self { wine, wine_server })
    }
}

#[derive(Debug)]
pub struct WinePrefix {
    wine: WineTool,
    prefix_path: PathBuf,
    wine_config: WineConfig,
    wine_server_child: Option<GroupChild>,
}

impl WinePrefix {
    pub fn new(wine: WineTool, prefix_path: PathBuf) -> Result<Self> {
        let wine_config = WineConfig::from_prefix(&prefix_path);

        let mut res = Self {
            wine,
            prefix_path,
            wine_config,
            wine_server_child: None,
        };

        debug!(
            "Creating a wine prefix at {:?} using {:?}",
            res.prefix_path, res.wine
        );

        std::fs::create_dir_all(&res.prefix_path).context("Creating wine context dir")?;
        res.kill_wine_server()
            .context("Kill any pre-existing wine servers")?;

        debug!("Running a wine server");

        res.wine_server_child = Some(
            res.wine_server()?
                .arg("-p") // persistent
                .arg("-f") // and foreground
                .group_spawn()?,
        );

        debug!("Running wineboot");
        res.wine()?.arg("wineboot").checked_status()?;

        // now that we have ran wineboot which should've created all the required paths we re-create WineConfig for it to update mapping caches
        res.wine_config = WineConfig::from_prefix(&res.prefix_path);

        Ok(res)
    }

    fn env(&self) -> Result<HashMap<String, String>> {
        Ok(HashMap::from([
            (
                "WINEPREFIX".to_string(),
                self.prefix_path
                    .canonicalize()?
                    .to_str()
                    .unwrap()
                    .to_string(),
            ),
            ("WINEDEBUG".to_string(), "-all".to_string()),
        ]))
    }

    fn wine(&self) -> Result<Command> {
        let mut command = Command::new(&self.wine.wine);
        command.envs(self.env()?);
        Ok(command)
    }

    fn wine_server(&self) -> Result<Command> {
        let mut command = Command::new(&self.wine.wine_server);
        command.envs(self.env()?);
        Ok(command)
    }

    fn kill_wine_server(&self) -> Result<()> {
        // ignore status of -k (it fails when there is no wineserver I think
        debug!("Killing any previously running wine server");
        self.wine_server()?
            .arg("-k")
            .status()
            .context("Running wineserver -k")?;

        debug!("Waiting for wineserver to die");

        self.wine_server()?
            .arg("-w")
            .checked_status()
            .context("Waiting for wineserver to die")?;

        Ok(())
    }

    pub fn command_inside<S: AsRef<OsStr>>(&self, program: S) -> Result<Command> {
        let mut command = self.wine()?;
        command.arg(program);
        Ok(command)
    }

    pub fn windows_path<S: AsRef<Path>>(&self, linux_path: S) -> Result<WinePath> {
        let linux_path = linux_path
            .as_ref()
            .absolutize()
            .context("Getting absolute path for passing it to to_wine_path")?;
        let path = self.wine_config.to_wine_path(linux_path)?;
        Ok(path)
    }
}

impl Drop for WinePrefix {
    fn drop(&mut self) {
        self.kill_wine_server().unwrap();
        if let Some(child) = &mut self.wine_server_child {
            child.wait().unwrap();
        }
    }
}
