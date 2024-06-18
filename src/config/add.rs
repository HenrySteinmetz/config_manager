use std::path::{Path, PathBuf};

use super::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::utils::{get_base_dir, ConfigResult};
use crate::{try_read_and_parse, try_rename, try_symlink, try_write_file};

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

    let link_string = theme_path.clone() + "/" + &name.clone();
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
