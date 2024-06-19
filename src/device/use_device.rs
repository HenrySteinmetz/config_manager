use crate::config::ConfigFile;
use crate::dependency::Dependency;
use crate::error::ConfigCliError;
use crate::try_read_and_parse;
use crate::utils::{get_base_dir, get_current_theme, ConfigResult};

pub fn use_device(name: String) -> ConfigResult<()> {
    let config_file_location = get_base_dir()? + &get_current_theme()? + "/dependencies.toml";
    let config_file = try_read_and_parse!(config_file_location.clone(), ConfigFile);
    let deps = config_file
        .device_bounds
        .into_iter()
        .filter(|x| x.0 == name)
        .map(|x| x.1.dependencies)
        .collect::<Vec<Vec<Dependency>>>();
    for dependency in deps.iter().flatten() {
        dependency.install()?;
    }
    Ok(())
}
