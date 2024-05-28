use crate::{error::ConfigCliError, try_git};
use std::process::Command;
use crate::{get_base_dir, get_current_theme, try_create_file, utils::ConfigResult};
use std::path::Path;

//TODO!
pub fn install_theme(url: String) -> ConfigResult<()> {
    Ok(())
}

pub fn set_url(url: String) -> ConfigResult<()> {
    let theme_dir = get_base_dir()? + &get_current_theme()?;
    let git_conf_location = theme_dir.clone() + "/.gitconfig";

    if !Path::new(&git_conf_location).exists() {
        try_create_file!(git_conf_location.clone());
        try_git!("init", &theme_dir);
        try_git!("remote add origin ".to_owned() + &url, &theme_dir);
    } else {
        try_git!("remote set-url origin ".to_owned() + &url, &theme_dir);
    }
    
    Ok(())
}

pub fn pull() -> ConfigResult<()> {
    try_git!("pull", get_base_dir()? + &get_current_theme()?);
    Ok(())
}

// TODO!
pub fn push() -> ConfigResult<()> {
    try_git!("push", get_base_dir()? + &get_current_theme()?);
    Ok(())
}
