use crate::{try_copy_recursive, try_delete, try_read_and_parse, try_rename, try_symlink, try_write_file};

use crate::dependency::Dependency;
use crate::error::ConfigCliError;
use crate::utils::{copy_dir_all, get_base_dir, ConfigResult};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub dependencies: Vec<Dependency>,
    pub symlink: PathBuf,
    pub conf_location: PathBuf,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub globals: Vec<Config>,
    pub device_bounds: Vec<(String, Config)>,
}

impl ConfigFile {
    pub fn empty() -> Self {
        Self {
            globals: vec![],
            device_bounds: vec![],
        }
    }
}

pub fn list_configs(theme: String, device: Option<String>) -> ConfigResult<Vec<Config>> {
    let theme = get_base_dir()? + &theme + "/configs.toml";
    let config_file = try_read_and_parse!(theme, ConfigFile);

    if device.is_some() {
        Ok(config_file
            .device_bounds
            .iter()
            .filter(|x| x.0 == device.clone().unwrap())
            .map(|x| x.1.clone())
            .collect())
    } else {
        let mut all_configs: Vec<Config> = config_file.globals;
        all_configs.extend(
            config_file
                .device_bounds
                .into_iter()
                .map(|x| x.1)
                .collect::<Vec<Config>>(),
        );
        Ok(all_configs)
    }
}

pub fn add_config(
    name: String,
    device: Option<String>,
    theme: String,
    file: PathBuf,
) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &theme;

    if !Path::new(&theme_path).exists() {
        return Err(ConfigCliError::InvalidThemeName(theme));
    }

    let config_file_path = theme_path.clone() + "/configs.toml";

    let config_file = try_read_and_parse!(config_file_path.clone(), ConfigFile);
    let mut config_file_clone = config_file.clone();

    let global_matching_configs: Vec<Config> = config_file
        .globals
        .into_iter()
        .filter(|conf| conf.name == name)
        .collect();

    let device_matching_configs: Vec<(String, Config)> = if device.is_some() {
        config_file
            .device_bounds
            .into_iter()
            .filter(|conf| conf.0 == device.clone().unwrap() && conf.1.name == name)
            .collect()
    } else {
        vec![]
    };

    if global_matching_configs.len() > 0 || device_matching_configs.len() > 0 {
        return Err(ConfigCliError::InvalidConfigName(name));
    }

    let link_string = theme_path + "/" + &name.clone();
    let link_path = Path::new(&link_string);

    let new_conf = Config {
        name,
        dependencies: vec![],
        symlink: file.clone(),
        conf_location: link_path.to_path_buf(),
        active: false,
    };

    if device.is_some() {
        config_file_clone
            .device_bounds
            .push((device.unwrap(), new_conf));
    } else {
        config_file_clone.globals.push(new_conf);
    }

    try_rename!(file.clone(), link_path);
    try_symlink!(link_path, file);

    try_write_file!(theme_path, &config_file_clone);

    Ok(())
}

pub fn remove_config(name: String, theme: String) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &theme;

    if !Path::new(&theme_path).exists() {
        return Err(ConfigCliError::InvalidThemeName(theme));
    }

    let config_file_path = theme_path.clone() + "/configs.toml";

    let config_file = try_read_and_parse!(config_file_path.clone(), ConfigFile);
    let mut config_file_clone = config_file.clone();

    let mut all_configs: Vec<Config> = config_file.globals;
    all_configs.extend(config_file.device_bounds.into_iter().map(|x| x.1));

    let config_to_remove: &Config = all_configs
        .iter()
        .filter(|conf| conf.name == name)
        .last()
        .ok_or::<ConfigCliError>(
        ConfigCliError::InvalidConfigName(name)
    )?;

    try_delete!(config_to_remove.symlink.clone());
    try_copy_recursive!(
        config_to_remove.conf_location.clone(),
        config_to_remove.symlink.clone()
    );

    config_file_clone.globals.retain(|conf| conf.name != name);
    config_file_clone
        .device_bounds
        .retain(|conf| conf.1.name != name);

    try_write_file!(config_file_path, &config_file_clone);

    Ok(())
}
