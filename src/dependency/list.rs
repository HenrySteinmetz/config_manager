use crate::config::{Config, ConfigFile};
use crate::error::ConfigCliError;
use crate::try_read_and_parse;
use crate::utils::{get_base_dir, ConfigResult};

pub fn list_dependencies(theme: String, config_name: String) -> ConfigResult<Vec<String>> {
    let theme = get_base_dir()? + &theme + "/configs.toml";

    let config_file = try_read_and_parse!(theme, ConfigFile);

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
