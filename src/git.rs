use crate::error::ConfigCliError;
use std::{io::Write, process::Command};
use crate::{get_base_dir, get_current_theme, try_create_file, try_write_file, utils::ConfigResult};
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
struct GitConfig {
    url: String,
}

//TODO!
pub fn install_theme(url: String) -> ConfigResult<()> {
    Ok(())
}

//TODO!
pub fn set_url(url: String) -> ConfigResult<()> {
    let theme_dir = get_base_dir()? + &get_current_theme()?;
    let git_conf_location = theme_dir.clone() + "/.gitconfig";

    if !Path::new(&git_conf_location).exists() {
        try_create_file!(git_conf_location.clone());
        match Command::new("sh").arg("-c").arg(format!(r#""cd {&theme_dir} && git init""#)).current_dir(&theme_dir).output() {
            Ok(x) => {
                if x.stderr.len() > 0 {
                    return Err(ConfigCliError::GitCommandError(std::str::from_utf8(&x.stderr).unwrap().to_owned()));
                }
            }
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        }
    }
    
    try_write_file!(Path::new(&git_conf_location), &GitConfig{ url });

    Ok(())
}

// TODO!
pub fn pull() -> ConfigResult<()> {
    Command::new("git").arg("pull")
    Ok(())
}

// TODO!
pub fn push() -> ConfigResult<()> {
    Ok(())
}
