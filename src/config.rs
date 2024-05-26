use crate::dependency::Dependency;
use crate::utils::{err, get_base_dir, ConfigResult, copy_dir_all};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub dependencies: Vec<Dependency>,
    pub symlink: PathBuf,
    pub conf_location: PathBuf,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub globals: Vec<Config>,
    pub device_bounds: Vec<(String, Config)>,
}

impl ConfigFile {
    pub fn empty() -> Self {
        Self {
            globals: vec![],
            device_bounds: vec![],
        }
    }
}

pub fn list_configs(theme: String, device: Option<String>) -> ConfigResult<Vec<Config>> {
    let theme = get_base_dir()? + &theme + "/configs.toml";
    let file_contents = std::fs::read(Path::new(&theme))?;
    let config_file: ConfigFile = toml::from_str(std::str::from_utf8(&file_contents)?)?;

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
               .collect::<Vec<Config>>());
        Ok(all_configs)
    }
}

pub fn add_config(
    name: String,
    device: Option<String>,
    theme: String,
    file: PathBuf,
) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &theme;

    if !Path::new(&theme_path).exists() {
        return err!("Invalid theme name!");
    }

    let config_file_path = theme_path.clone() + "/configs.toml";
    let file_contents = std::fs::read(config_file_path.clone())?;

    let config_file: ConfigFile = toml::from_str(std::str::from_utf8(&file_contents)?)?;
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
        return Err("A config with selected name already exists."
            .to_owned()
            .into());
    }

    let link_string = theme_path + "/" + &name.clone();
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

    std::fs::rename(file.clone(), link_path)?;
    std::os::unix::fs::symlink(link_path, file)?;

    let mut file_handle = std::fs::OpenOptions::new()
        .write(true)
        .open(config_file_path)?;

    let config_string = toml::to_string(&config_file_clone)?;
    file_handle.write_all(config_string.as_bytes())?;
    file_handle.flush()?;

    Ok(())
}

pub fn remove_config(name: String, theme: String) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &theme;

    if !Path::new(&theme_path).exists() {
        return err!("Invalid theme name!");
    }

    let config_file_path = theme_path.clone() + "/configs.toml";
    let file_contents = std::fs::read(config_file_path.clone())?;

    let config_file: ConfigFile = toml::from_str(std::str::from_utf8(&file_contents)?)?;
    let mut config_file_clone = config_file.clone();

    if !Path::new(&theme_path).exists() {
        return Err("Invalid theme name!".to_owned().into());
    }
    
    let mut all_configs: Vec<Config> = config_file.globals;
    all_configs.extend(
        config_file
           .device_bounds
           .into_iter()
           .map(|x| x.1));

    let config_to_remove: &Config = all_configs
        .iter()
        .filter(|conf| conf.name == name).last()
        .ok_or::<Box<dyn std::error::Error + 'static>>("Invalid config name!".to_owned().into())?;
    
    std::fs::remove_file(config_to_remove.symlink.clone())?;
    copy_dir_all(config_to_remove.conf_location.clone(), config_to_remove.symlink.clone())?;

    config_file_clone.globals.retain(|conf| conf.name != name);
    config_file_clone.device_bounds.retain(|conf| conf.1.name != name);

    let mut file_handle = std::fs::OpenOptions::new()
        .write(true)
        .open(config_file_path)?;

    let config_string = toml::to_string(&config_file_clone)?;
    file_handle.write_all(config_string.as_bytes())?;
    file_handle.flush()?;

    Ok(())
}
