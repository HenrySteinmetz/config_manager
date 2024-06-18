use crate::error::ConfigCliError;
use crate::{get_base_dir, ConfigResult};

pub fn list_themes() -> ConfigResult<Vec<String>> {
    let mut ret: Vec<String> = vec![];
    let theme_path = get_base_dir()?;

    let read_dir = match std::fs::read_dir(theme_path) {
        Ok(dir) => dir,
        Err(err) => return Err(ConfigCliError::FsReadError(err)),
    };

    for theme in read_dir {
        let theme_path = match theme {
            Ok(path) => path.path(),
            Err(err) => return Err(ConfigCliError::FsReadError(err)),
        };
        if theme_path.clone().is_dir() {
            ret.push(theme_path.to_str().unwrap().to_owned());
        }
    }

    Ok(ret)
}
