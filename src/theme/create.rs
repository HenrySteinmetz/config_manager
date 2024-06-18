use std::path::Path;

use crate::config::ConfigFile;
use crate::dependency::DependencyFile;
use crate::error::ConfigCliError;
use crate::ConfigResult;
use crate::{copy_dir_all, get_base_dir};
use crate::{try_copy_recursive, try_create_file, try_write_file};

pub fn create_theme(name: String, base: Option<String>) -> ConfigResult<()> {
    let theme_path = get_base_dir()? + &name;

    if Path::new(&theme_path).exists() {
        return Err(ConfigCliError::InvalidThemeName(name));
    }

    match std::fs::create_dir(theme_path.clone()) {
        Ok(_) => (),
        Err(err) => return Err(ConfigCliError::FileCreationError(err)),
    }

    match base {
        Some(base) => {
            let read_dir = match std::fs::read_dir(get_base_dir()? + &base) {
                Ok(dir) => dir,
                Err(err) => return Err(ConfigCliError::FsReadError(err)),
            };
            for file in read_dir {
                let file = match file {
                    Ok(file) => file,
                    Err(err) => return Err(ConfigCliError::FsReadError(err)),
                };
                try_copy_recursive!(file.path(), theme_path.clone());
            }
        }
        None => {
            try_create_file!(theme_path.clone() + "/dependencies.toml");
            try_write_file!(
                theme_path.clone() + "/dependencies.toml",
                &DependencyFile::default()
            );

            try_create_file!(theme_path.clone() + "/config.toml");
            try_write_file!(theme_path.clone() + "/config.toml", &ConfigFile::default());
        }
    }
    Ok(())
}
