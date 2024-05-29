use crate::error::ConfigCliError;
use crate::{
    config::ConfigFile, get_base_dir, get_current_theme, try_read_and_parse, ConfigResult,
};
use crate::{dependency, try_write_file};
use itertools::Itertools;

// TODO
pub fn add_device(name: String) -> ConfigResult<()> {
    let config_file_location = get_base_dir()? + &get_current_theme()? + "/configs.toml";

    let config_file = try_read_and_parse!(config_file_location.clone(), ConfigFile);
    let mut config_file_clone = config_file.clone();

    config_file_clone.devices.push(name);

    try_write_file!(&config_file_location, &config_file_clone);
    Ok(())
}

// TODO
pub fn list_devices() -> ConfigResult<Vec<String>> {
    let config_file_location = get_base_dir()? + &get_current_theme()? + "/configs.toml";
    Ok(
        try_read_and_parse!(config_file_location.clone(), ConfigFile)
            .device_bounds
            .into_iter()
            .map(|x| x.0)
            .dedup()
            .collect(),
    )
}

// TODO
pub fn remove_device(name: String) -> ConfigResult<()> {
    let config_file_location = get_base_dir()? + &get_current_theme()? + "/configs.toml";
    let mut config_file = try_read_and_parse!(config_file_location.clone(), ConfigFile);
    config_file.device_bounds = config_file
        .device_bounds
        .into_iter()
        .filter(|x| x.0 != name)
        .collect();

    try_write_file!(&config_file_location, &config_file);

    Ok(())
}

// TODO
pub fn use_device(name: String) -> ConfigResult<()> {
    let config_file_location = get_base_dir()? + &get_current_theme()? + "/dependencies.toml";
    let config_file = try_read_and_parse!(config_file_location.clone(), ConfigFile);
    let deps = config_file
        .devices
        .into_iter()
        .filter(|x| x.0 == name)
        .map(|x| x.1)
        .collect::<Vec<Vec<String>>>();
    for dependency in deps {
        dependency.install()
    }
    Ok(())
}
