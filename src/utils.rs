use std::ops::Deref;
use std::path::Path;
use serde::{Deserialize, Serialize};

pub type ConfigResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

macro_rules! err {
    ($e: expr) => {
        Err($e.to_owned().into())
    };
}
// Allows the err macro to be used everywhere in the crate
pub(crate) use err;

pub fn get_base_dir() -> ConfigResult<String> {
    #[allow(deprecated)]
    match std::env::home_dir() {
        Some(x) => {
            Ok(x.deref().as_os_str().to_str().unwrap().to_owned() + "/.local/share/configmanager/")
        }
        None => err!("Couldn't get home directory from enviorment variables!"),
    }
}

#[derive(Serialize, Deserialize)]
pub struct CurrentTheme {
    pub current_theme: String
}

pub fn get_current_theme() -> ConfigResult<String> {
    let base_dir = get_base_dir()? + "current_theme.toml";
    if !Path::new(&base_dir).exists() {
        std::fs::File::create(&base_dir)?;
        return err!("No Theme selected! Please use a theme or create a new one using the theme subcommand!");
    }
    let file_raw = std::fs::read(base_dir)?;
    let file_contents = std::str::from_utf8(&file_raw)?;
    Ok(toml::from_str::<CurrentTheme>(file_contents)?.current_theme)
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
