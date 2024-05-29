use crate::config::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::utils::{get_base_dir, ConfigResult};
use crate::{try_read_and_parse, try_write_file};

use std::io::Write;
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dependency(String);

impl Dependency {
    pub fn install(&self) -> ConfigResult<()> {
        let output = match Command::new("yay").arg("-Q").arg(self.0.clone()).output() {
            Ok(cli) => cli,
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        };
        if output.status.code() == Some(0) {
            return Ok(());
        }
        let install_output = match Command::new("yay").arg("-S").arg(self.0.clone()).output() {
            Ok(cli) => cli,
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        };

        match install_output.status.code() {
            Some(0) => Ok(()),
            _ => Err(ConfigCliError::GitCommandError(
                install_output.status.to_string(),
            )),
        }
    }
}

impl TryFrom<String> for Dependency {
    type Error = ConfigCliError;
    fn try_from(value: String) -> ConfigResult<Self> {
        let output = match Command::new("yay").arg("-Ssq").arg(value.clone()).output() {
            Ok(cli) => cli.stdout,
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        };

        let results: Vec<String> = match std::str::from_utf8(&output) {
            Ok(s) => s.to_owned().lines().map(|x| x.to_owned()).collect(),
            Err(err) => return Err(ConfigCliError::StringConversionError(err)),
        };

        if results.contains(&value) {
            Ok(Dependency(value))
        } else {
            Err(ConfigCliError::NoPackageWithName(value))
        }
    }
}

impl Into<Dependency> for (String, Dependency) {
    fn into(self) -> Dependency {
        self.1
    }
}

#[derive(Serialize, Deserialize)]
pub struct DependencyFile {
    pub globals: Vec<Dependency>,
    pub config_bounds: Vec<(String, Dependency)>,
}

impl DependencyFile {
    pub fn empty() -> Self {
        Self {
            globals: vec![],
            config_bounds: vec![],
        }
    }
}

pub fn remove_dependency(theme: String, dependency: String) -> ConfigResult<()> {
    let path = get_base_dir()? + &theme + "/dependencies.toml";
    let file_contents = try_read_and_parse!(path.clone(), DependencyFile);

    let mut all_dependencies = file_contents.globals.clone();
    let config_dependencies: Vec<Dependency> = file_contents
        .config_bounds
        .iter()
        .map(|x| Into::<DependencyWrapper>::into(x))
        .collect();
    all_dependencies.extend(config_dependencies.clone());

    let dependency = Dependency(dependency);

    if config_dependencies.contains(&dependency) {
        let new_file_contents: DependencyFile = DependencyFile {
            config_bounds: file_contents
                .config_bounds
                .into_iter()
                .filter(|x| x.1 .0 != dependency.0)
                .collect(),
            globals: file_contents.globals,
        };

        try_write_file!(path, &new_file_contents);

        Ok(())
    } else if all_dependencies.contains(&dependency) {
        let new_file_contents: DependencyFile = DependencyFile {
            config_bounds: file_contents.config_bounds,
            globals: file_contents
                .globals
                .into_iter()
                .filter(|x| x.0 != dependency.0)
                .collect(),
        };

        try_write_file!(path, &new_file_contents);

        Ok(())
    } else {
        Err(ConfigCliError::InvalidConfigName(dependency.0))
    }
}

pub fn add_dependency(
    theme: String,
    config: Option<String>,
    dependency: String,
) -> ConfigResult<()> {
    let path = get_base_dir()? + &theme + "/dependencies.toml";
    let mut file_contents = try_read_and_parse!(path.clone(), DependencyFile);
    let dependencies = &mut file_contents.globals;
    let config_dependencies: Vec<DependencyWrapper> = file_contents
        .config_bounds
        .iter()
        .map(|x| Into::<DependencyWrapper>::into(x))
        .collect();

    let dependency = Dependency(dependency);

    if config.is_some()
        && config_dependencies
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<String>>()
            .as_slice()
            .contains(config.as_ref().unwrap())
    {
        dependencies.extend(
            config_dependencies
                .into_iter()
                .map(|x| Into::<Dependency>::into(x))
                .collect::<Vec<Dependency>>(),
        );
    }

    if dependencies.contains(&dependency) {
        return Err(ConfigCliError::InvalidDependencyName(dependency.0));
    }

    if config.is_some() {
        file_contents
            .config_bounds
            .push((config.unwrap(), dependency));
    } else {
        file_contents.globals.push(dependency);
    }

    try_write_file!(path, &file_contents);
    Ok(())
}

struct DependencyWrapper(String, Dependency);

impl FromIterator<DependencyWrapper> for Vec<Dependency> {
    fn from_iter<T: IntoIterator<Item = DependencyWrapper>>(iter: T) -> Self {
        iter.into_iter().map(|x| x.1).collect()
    }
}

impl Into<Dependency> for DependencyWrapper {
    fn into(self) -> Dependency {
        self.1
    }
}

impl Into<DependencyWrapper> for &(String, Dependency) {
    fn into(self) -> DependencyWrapper {
        DependencyWrapper(self.0.clone(), self.1.clone())
    }
}

impl Into<DependencyWrapper> for (String, Dependency) {
    fn into(self) -> DependencyWrapper {
        DependencyWrapper(self.0, self.1)
    }
}

pub fn list_dependencies(theme: String, config_name: String) -> ConfigResult<Vec<String>> {
    let theme = get_base_dir()? + &theme + "/configs.toml";

    let config_file = try_read_and_parse!(theme, ConfigFile);

    let mut all_deps: Vec<Config> = config_file.globals;
    all_deps.extend(
        config_file
            .device_bounds
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<Config>>(),
    );

    Ok(all_deps
        .into_iter()
        .filter(|x| x.name == config_name)
        .map(|x| x.dependencies.into_iter().map(|x| x.0).collect::<String>())
        .collect())
}
