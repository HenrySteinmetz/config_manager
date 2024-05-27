use crate::error::ConfigCliError;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::path::Path;

pub type ConfigResult<T> = Result<T, ConfigCliError>;

pub fn get_base_dir() -> ConfigResult<String> {
    #[allow(deprecated)]
    let home_dir = std::env::home_dir().ok_or(ConfigCliError::UnableToFindHomeDir)?;
    Ok(home_dir.deref().as_os_str().to_str().unwrap().to_owned() + "/.local/share/configmanager/")
}

#[derive(Serialize, Deserialize)]
pub struct CurrentTheme {
    pub current_theme: String,
}

#[macro_export]
macro_rules! try_read_file {
    ($path: expr) => {
        match std::fs::read($path) {
            Ok(file) => file,
            Err(err) => return Err(ConfigCliError::FsReadError(err))
        }
    };
}

#[macro_export]
macro_rules! try_parse_toml {
    ($content: expr, $type: ty) => {
        match toml::from_str::<$type>(&$content) {
            Ok(toml) => toml,
            Err(err) => return Err(ConfigCliError::DeserializeError(err))
        }
    };
}


macro_rules! try_read_and_parse {
    ($path: expr, $type: ty) => {
        {
            use crate::{try_read_file, try_parse_toml};

            let file_contents = try_read_file!($path);
            let string = match std::str::from_utf8(&file_contents) {
                Ok(string) => string,
                Err(err) => return Err(ConfigCliError::StringConversionError(err)),
            }; 
            try_parse_toml!(string, $type)
        }
    };
}

macro_rules! try_create_file {
    ($path: expr) => {
        match std::fs::File::create(&$path) {
            Ok(_) => (),
            Err(err) => return Err(ConfigCliError::FileCreationError(err))
        }
    };
}

#[macro_export]
macro_rules! try_symlink {
    ($location:expr, $link:expr) => {
        match std::os::unix::fs::symlink($location, $link) {
            Ok(_) => (),
            Err(err) => return Err(ConfigCliError::SymlinkError(err)),
        }
    };
}


#[macro_export]
macro_rules! try_rename {
    ($src:expr, $dst:expr) => {
        match std::fs::rename($src, $dst) {
            Ok(_) => (),
            Err(err) => return Err(ConfigCliError::RenameError(err)),
        }
    };
}

#[macro_export]
macro_rules! try_write_file {
    ($location:expr, $content:expr) => {
        {
            let mut file_handle = match std::fs::OpenOptions::new().write(true).open($location) {
                Ok(f) => f,
                Err(err) => return Err(ConfigCliError::FsWriteError(err)),
            };
            let content_string = match toml::to_string($content) {
                Ok(s) => s,
                Err(err) => return Err(ConfigCliError::SerializeError(err)),
            };

            match file_handle.write_all(content_string.as_bytes()) {
                Ok(_) => (),
                Err(err) => return Err(ConfigCliError::FsWriteError(err)),
            }

            match file_handle.flush() {
                Ok(_) => (),
                Err(err) => return Err(ConfigCliError::FsWriteError(err)),
            }

        }
    };
}

#[macro_export]
macro_rules! try_delete {
    ($path: expr) => {
        match std::fs::remove_file($path) {
            Ok(_) => (),
            Err(err) => return Err(ConfigCliError::DeleteError(err)),
        }
    };
}

#[macro_export]
macro_rules! try_copy_recursive {
    ($src: expr, $dst: expr) => {
        match copy_dir_all($src, $dst) {
            Ok(_) => (),
            Err(err) => return Err(ConfigCliError::CopyError(err)),
        }
    };
}

#[macro_export]
macro_rules! try_delete_recursive {
    ($loc: expr) => {
        match std::fs::remove_dir_all($loc) {
            Ok(_) => (),
            Err(err) => return Err(ConfigCliError::DeleteError(err)),
        }
    };
}

// This allows the macros to be used out side of the file
pub(crate) use try_create_file; 
pub(crate) use try_read_and_parse; 
pub(crate) use try_symlink; 
pub(crate) use try_rename; 
pub(crate) use try_write_file;
pub(crate) use try_copy_recursive;
pub(crate) use try_delete_recursive;



pub fn get_current_theme() -> ConfigResult<String> {
    let base_dir = get_base_dir()? + "current_theme.toml";

    if !Path::new(&base_dir).exists() {
        try_create_file!(base_dir);
        return Err(ConfigCliError::NoThemeSelecected);
    }
    Ok(try_read_and_parse!(base_dir, CurrentTheme).current_theme)
}

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
