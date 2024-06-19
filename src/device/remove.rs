use crate::config::ConfigFile;
use crate::error::ConfigCliError;
use crate::utils::{get_base_dir, get_current_theme, ConfigResult};
use crate::{try_read_and_parse, try_write_file};

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
