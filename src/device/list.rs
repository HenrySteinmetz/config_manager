use itertools::Itertools;

use crate::config::ConfigFile;
use crate::error::ConfigCliError;
use crate::try_read_and_parse;
use crate::utils::{get_base_dir, get_current_theme, ConfigResult};

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
