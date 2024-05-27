use crate::config::{Config, ConfigFile};
use crate::utils::{err, get_base_dir, ConfigResult};

use std::io::Write;
use std::path::Path;
use std::process::Command;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dependency(String);

impl TryFrom<String> for Dependency {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: String) -> ConfigResult<Self> {
        let output = Command::new("yay")
            .arg("-Ssq")
            .arg(value.clone())
            .output()?
            .stdout;
        let results: Vec<String> = std::str::from_utf8(&output)?
            .to_owned()
            .lines()
            .map(|x| x.to_owned())
            .collect();

        if results.contains(&value) {
            Ok(Dependency(value))
        } else {
            err!("Couldn't find an exact match in the repositories!")
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
    globals: Vec<Dependency>,
    config_bounds: Vec<(String, Dependency)>,
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
    let file_contents: DependencyFile =
        toml::from_str(std::str::from_utf8(&std::fs::read(path.clone())?)?)?;

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

        let mut file_handle = std::fs::OpenOptions::new().write(true).open(path)?;
        let string_contents = toml::to_string(&new_file_contents)?;
        file_handle.write_all(string_contents.as_bytes())?;
        file_handle.flush()?;

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

        let mut file_handle = std::fs::OpenOptions::new().write(true).open(path)?;
        let string_contents = toml::to_string(&new_file_contents)?;
        file_handle.write_all(string_contents.as_bytes())?;
        file_handle.flush()?;

        Ok(())
    } else {
        err!("Couldn't find the dependency you were trying to remove!")
    }
}

pub fn add_dependency(
    theme: String,
    config: Option<String>,
    dependency: String,
) -> ConfigResult<()> {
    let path = get_base_dir()? + &theme + "/dependencies.toml";
    let mut file_contents: DependencyFile =
        toml::from_str(std::str::from_utf8(&std::fs::read(path.clone())?)?)?;
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
        return err!("Dependency was already present!");
    }

    if config.is_some() {
        file_contents
            .config_bounds
            .push((config.unwrap(), dependency));
    } else {
        file_contents.globals.push(dependency);
    }

    let mut file_handle = std::fs::OpenOptions::new().write(true).open(path)?;
    let string_contents = toml::to_string(&file_contents)?;
    file_handle.write_all(string_contents.as_bytes())?;
    file_handle.flush()?;
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

    let file_contents = std::fs::read(Path::new(&theme))?;
    let config_file: ConfigFile = toml::from_str(std::str::from_utf8(&file_contents)?)?;

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
