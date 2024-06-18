use std::path::{Path, PathBuf};

use crate::config::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::utils::{copy_dir_all, get_base_dir};
use crate::{
    try_copy_recursive, try_delete, try_delete_recursive, try_read_and_parse, ConfigResult,
};

pub fn remove_theme(name: String) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &name;

    let configs_to_remove = try_read_and_parse!(
        Path::new(&(theme_path.clone() + "configs.toml")),
        ConfigFile
    );

    let mut all_configs: Vec<Config> = configs_to_remove.globals;

    all_configs.extend(
        configs_to_remove
            .device_bounds
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<Config>>(),
    );

    let config_loactions: Vec<PathBuf> = all_configs
        .clone()
        .into_iter()
        .map(|x| x.conf_location)
        .collect();

    let mut saved_configs: Vec<Config> = vec![];

    let read_dir = match std::fs::read_dir(get_base_dir()?) {
        Ok(dir) => dir,
        Err(err) => return Err(ConfigCliError::FsReadError(err)),
    };

    for theme in read_dir {
        let theme_path = match theme {
            Ok(path) => path.path(),
            Err(err) => return Err(ConfigCliError::FsReadError(err)),
        };
        let configs = try_read_and_parse!(theme_path, ConfigFile);

        saved_configs.extend(
            configs
                .globals
                .into_iter()
                .filter(|x| config_loactions.contains(&x.conf_location))
                .collect::<Vec<Config>>(),
        );
    }

    let unsaved_configs = all_configs
        .into_iter()
        .filter(|x| saved_configs.contains(&x))
        .collect::<Vec<Config>>();

    // Put the configs that aren't used anywhere else back into their original location
    for unsaved_config in unsaved_configs {
        if unsaved_config.active {
            try_delete!(unsaved_config.symlink.clone());
        }

        try_copy_recursive!(unsaved_config.conf_location, unsaved_config.symlink);
    }

    try_delete_recursive!(theme_path);
    Ok(())
}
