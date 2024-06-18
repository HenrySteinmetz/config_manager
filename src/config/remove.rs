use std::path::Path;

use super::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::utils::{get_base_dir, ConfigResult};
use crate::{copy_dir_all, try_copy_recursive, try_delete, try_read_and_parse, try_write_file};

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

    let config_to_remove: &Config =
        all_configs
            .iter()
            .filter(|conf| conf.name == name)
            .last()
            .ok_or::<ConfigCliError>(ConfigCliError::InvalidConfigName(name.clone()))?;

    try_delete!(config_to_remove.symlink.clone());
    try_copy_recursive!(
        config_to_remove.conf_location.clone(),
        config_to_remove.symlink.clone()
    );

    config_file_clone
        .globals
        .retain(|conf| conf.name != name.clone());
    config_file_clone
        .device_bounds
        .retain(|conf| conf.1.name != name);

    try_write_file!(config_file_path, &config_file_clone);

    Ok(())
}
