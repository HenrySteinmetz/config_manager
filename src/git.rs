use crate::{dependency::DependencyFile, error::ConfigCliError, try_git, try_read_and_parse};
use std::process::Command;
use crate::{get_base_dir, get_current_theme, utils::ConfigResult};

pub fn install_theme(url: String) -> ConfigResult<()> {
    let base_dir = get_base_dir()?;
    try_git!("clone ".to_owned() + &url, &base_dir);
    
    let dependencies = try_read_and_parse!(base_dir + "dependencies.toml", DependencyFile);
    for dependency in dependencies.globals {
        dependency.install()?;
    }
    Ok(())
}

pub fn git_init(url: String) -> ConfigResult<()> {
    let theme_dir = get_base_dir()? + &get_current_theme()?;
    try_git!("init", &theme_dir);
    try_git!("remote add origin ".to_owned() + &url, &theme_dir);
    Ok(())
}

pub fn set_url(url: String) -> ConfigResult<()> {
    let theme_dir = get_base_dir()? + &get_current_theme()?;
    try_git!("remote set-url origin ".to_owned() + &url, &theme_dir);
    
    Ok(())
}

pub fn pull() -> ConfigResult<()> {
    try_git!("pull", get_base_dir()? + &get_current_theme()?);
    Ok(())
}

// TODO!
pub fn push(commit_message: Option<String>) -> ConfigResult<()> {
    let theme_dir = get_base_dir()? + &get_current_theme()?;
    try_git!("add .", &theme_dir);
    try_git!("commit -m ".to_owned() + &commit_message.unwrap_or("Automated commit from config_manager".to_owned()), &theme_dir);
    try_git!("push", &theme_dir);
    Ok(())
}
