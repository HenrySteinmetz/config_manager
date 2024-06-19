mod cli;
mod config;
mod dependency;
mod device;
mod error;
mod git;
mod print;
mod theme;
mod utils;

use cli::ConfigCli;
use config::Config;
use dependency::Dependency;
use utils::*;

use config::{add_config, list_configs, remove_config};
use dependency::{add_dependency, list_dependencies, remove_dependency};
use device::{list_devices, remove_device, use_device};
use git::*;
use theme::*;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
enum ThemeStatus {
    InProgress,
    Finished,
    GitCommited,
}

#[derive(Serialize, Deserialize)]
struct Device(String);

#[derive(Serialize, Deserialize)]
struct Theme {
    name: String,
    device: Device,
    global_dependencies: Vec<Dependency>,
    configs: Vec<Config>,
    status: ThemeStatus,
}

#[derive(Debug)]
enum CommandResult {
    DependencyThemeList(ConfigResult<Vec<String>>),
    ConfigList(ConfigResult<Vec<Config>>),
    AddRemove(ConfigResult<()>),
}

fn main() -> ConfigResult<()> {
    let config_cli = ConfigCli::parse();
    let base_dir: String = get_base_dir()?;

    match Path::exists(Path::new(&base_dir)) {
        true => (),
        false => match std::fs::create_dir(base_dir.clone()) {
            Ok(_) => (),
            Err(x) => panic!("{}", x),
        },
    }

    let current_theme_path = &(base_dir + "/current_theme.toml");
    let current_theme_path = Path::new(current_theme_path);
    match Path::exists(current_theme_path) {
        true => (),
        false => match std::fs::File::create(current_theme_path) {
            Ok(_) => (),
            Err(x) => panic!("{}", x),
        },
    }

    use cli::ConfigSubCommands::*;
    let options = config_cli.command;
    let result: CommandResult = match options {
        Dependency { action, .. } => {
            use cli::DependencyActions::*;
            let theme_name = get_current_theme()?;
            match action {
                Remove { dependency_name } => {
                    CommandResult::AddRemove(remove_dependency(theme_name, dependency_name))
                }
                Add {
                    dependency_name,
                    config_name,
                } => CommandResult::AddRemove(add_dependency(
                    theme_name,
                    config_name,
                    dependency_name,
                )),
                List { config_name } => {
                    CommandResult::DependencyThemeList(list_dependencies(config_name, theme_name))
                }
            }
        }
        Config { action, .. } => {
            use cli::ConfigActions::*;
            let theme_name = get_current_theme()?;
            match action {
                Remove { config_name } => {
                    CommandResult::AddRemove(remove_config(config_name, theme_name))
                }
                Add {
                    config_name,
                    file,
                    device_name,
                } => CommandResult::AddRemove(add_config(
                    config_name,
                    device_name.clone(),
                    theme_name,
                    file.to_path_buf(),
                )),
                List { device_name } => {
                    CommandResult::ConfigList(list_configs(theme_name, device_name))
                }
            }
        }
        Device { action } => {
            use cli::DeviceActions::*;
            match action {
                Remove { name } => CommandResult::AddRemove(remove_device(name)),
                Use { name } => CommandResult::AddRemove(use_device(name)),
                List => CommandResult::DependencyThemeList(list_devices()),
            }
        }
        Theme { action, .. } => {
            use cli::ThemeActions::*;
            match action {
                Remove { name } => CommandResult::AddRemove(remove_theme(name)),
                Create { name, base } => CommandResult::AddRemove(create_theme(name, base)),
                Use {
                    name,
                    force,
                    device,
                } => CommandResult::AddRemove(use_theme(name, force, device)),
                List => CommandResult::DependencyThemeList(list_themes()),
            }
        }
        Git { action, .. } => {
            use cli::GitActions::*;
            match action {
                SetUrl { url } => CommandResult::AddRemove(set_url(url)),
                InstallTheme { url } => CommandResult::AddRemove(install_theme(url)),
                Push { commit_message } => CommandResult::AddRemove(push(commit_message)),
                Pull => CommandResult::AddRemove(pull()),
            }
        }
    };
    result.print();

    Ok(())
}
