use crate::config::{Config, ConfigFile};
use crate::dependency::DependencyFile;
use crate::utils::{copy_dir_all, err, get_base_dir, CurrentTheme};
use crate::ConfigResult;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn create_theme(name: String, base: Option<String>) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &name;

    if Path::new(&theme_path).exists() {
        return err!("A theme with the same name already exists!");
    }

    std::fs::create_dir(theme_path.clone())?;

    match base {
        Some(base) => {
            for file in std::fs::read_dir(get_base_dir()? + &base)? {
                copy_dir_all(file?.path(), theme_path.clone())?;
            }
        }
        None => {
            let mut dependency_file =
                std::fs::File::create(theme_path.clone() + "/dependencies.toml")?;
            let empty_dependency_file = DependencyFile::empty();
            let string_dependency_file = toml::to_string(&empty_dependency_file)?;
            dependency_file.write(string_dependency_file.as_bytes())?;

            let mut config_file = std::fs::File::create(theme_path + "/configs.toml")?;
            let empty_config_file = ConfigFile::empty();
            let string_config_file = toml::to_string(&empty_config_file)?;
            config_file.write(string_config_file.as_bytes())?;
        }
    }
    Ok(())
}

pub fn list_themes() -> ConfigResult<Vec<String>> {
    let mut ret: Vec<String> = vec![];
    let theme_path = get_base_dir()?;

    for theme in std::fs::read_dir(theme_path)? {
        let theme_path = theme?.path();
        if theme_path.clone().is_dir() {
            ret.push(theme_path.to_str().unwrap().to_owned());
        }
    }

    Ok(ret)
}

pub fn remove_theme(name: String) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &name;

    let config_file_contents =
        std::fs::read(Path::new(&(theme_path.clone() + "configs.toml")))?;
    let configs_to_remove: ConfigFile =
        toml::from_str(std::str::from_utf8(&config_file_contents)?)?;

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

    for theme in std::fs::read_dir(get_base_dir()?)? {
        let theme_path = theme?.path();
        let configs: ConfigFile =
            toml::from_str(std::str::from_utf8(&std::fs::read(theme_path)?)?)?;

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
            std::fs::remove_file(unsaved_config.symlink.clone())?;
        }

        copy_dir_all(unsaved_config.conf_location, unsaved_config.symlink)?;
    }

    Ok(std::fs::remove_dir_all(theme_path)?)
}

fn apply_config(config: &Config, force: &bool) -> ConfigResult<()> {
    if config.conf_location.is_file() && force == &false {
       return err!(format!("{} is an already used file location. You can overwrite it with the --force flag.", config.conf_location.to_string_lossy())); 
    }
    if config.conf_location.is_dir() && force == &false {
       return err!(format!("{} is an already used directory location. You can overwrite it with the --force flag.", config.conf_location.to_string_lossy())); 
    }
    copy_dir_all(config.conf_location.clone(), config.symlink.clone())?;
    Ok(())
}

// Change the current theme file to the new theme
fn change_current_theme(name: String) -> ConfigResult<()> {
    let current_theme_path = get_base_dir()? + "current_theme.toml";

    if !Path::new(&current_theme_path).exists() {
        File::create(current_theme_path.clone())?;
    }

    let mut file = std::fs::OpenOptions::new().write(true).open(current_theme_path)?;
    let contents = toml::to_string(&CurrentTheme { current_theme: name })?;

    file.write_all(contents.as_bytes())?;
    file.flush()?;

    Ok(())
}

pub fn use_theme(name: String, force: bool, device: Option<String>) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &name;
    
    if !Path::new(&theme_path).exists() {
        return err!("Invalid theme name!");
    }

    let config_file_path = theme_path.clone() + "/configs.toml";
    let file_contents = std::fs::read(config_file_path)?;
    let config_file: ConfigFile = toml::from_str(std::str::from_utf8(&file_contents)?)?;

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
