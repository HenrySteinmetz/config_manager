use std::path::Path;

use crate::config::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::{
    copy_dir_all, get_base_dir, try_copy_recursive, try_create_file, try_read_and_parse,
    try_write_file, ConfigResult, CurrentTheme,
};

fn apply_config(config: &Config, force: &bool) -> ConfigResult<()> {
    if config.conf_location.is_file() && force == &false {
        return Err(ConfigCliError::InvalidConfigLocation(
            config.conf_location.to_string_lossy().to_string(),
        ));
    }
    if config.conf_location.is_dir() && force == &false {
        return Err(ConfigCliError::ConfigLocationUsed(
            config.conf_location.to_string_lossy().to_string(),
        ));
    }
    try_copy_recursive!(config.conf_location.clone(), config.symlink.clone());
    Ok(())
}

// Change the current theme file to the new theme
fn change_current_theme(name: String) -> ConfigResult<()> {
    let current_theme_path = get_base_dir()? + "current_theme.toml";

    if !Path::new(&current_theme_path).exists() {
        try_create_file!(current_theme_path.clone());
    }

    try_write_file!(
        current_theme_path.clone(),
        &CurrentTheme {
            current_theme: name
        }
    );

    Ok(())
}

pub fn use_theme(name: String, force: bool, device: Option<String>) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &name;

    if !Path::new(&theme_path).exists() {
        return Err(ConfigCliError::InvalidThemeName(name));
    }

    let config_file_path = theme_path.clone() + "/configs.toml";
    let config_file = try_read_and_parse!(config_file_path, ConfigFile);

    change_current_theme(name)?;

    for config in config_file.globals {
        apply_config(&config, &force)?;
    }

    if device.is_some() {
        let device_configs: Vec<Config> = config_file
            .device_bounds
            .iter()
            .filter(|x| x.0 == device.clone().unwrap())
            .map(|x| x.1.clone())
            .collect();

        for config in device_configs {
            apply_config(&config, &force)?;
        }
    }

    Ok(())
}
