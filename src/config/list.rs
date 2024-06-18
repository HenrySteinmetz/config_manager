use super::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::{get_base_dir, try_read_and_parse, ConfigResult};

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
