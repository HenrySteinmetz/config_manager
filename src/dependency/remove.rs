use super::{Dependency, DependencyFile, DependencyWrapper};
use crate::error::ConfigCliError;
use crate::utils::{get_base_dir, ConfigResult};
use crate::{try_read_and_parse, try_write_file};

pub fn remove_dependency(theme: String, dependency: String) -> ConfigResult<()> {
    let path = get_base_dir()? + &theme + "/dependencies.toml";
    let file_contents = try_read_and_parse!(path.clone(), DependencyFile);

    let mut all_dependencies = file_contents.globals.clone();
    let config_dependencies: Vec<Dependency> = file_contents
        .config_bounds
        .iter()
        .map(|x| Into::<DependencyWrapper>::into(x))
        .collect();
    all_dependencies.extend(config_dependencies.clone());

    let dependency = Dependency(dependency);

    if config_dependencies.contains(&dependency) {
        let new_file_contents: DependencyFile = DependencyFile {
            config_bounds: file_contents
                .config_bounds
                .into_iter()
                .filter(|x| x.1 .0 != dependency.0)
                .collect(),
            globals: file_contents.globals,
        };

        try_write_file!(path, &new_file_contents);

        Ok(())
    } else if all_dependencies.contains(&dependency) {
        let new_file_contents: DependencyFile = DependencyFile {
            config_bounds: file_contents.config_bounds,
            globals: file_contents
                .globals
                .into_iter()
                .filter(|x| x.0 != dependency.0)
                .collect(),
        };

        try_write_file!(path, &new_file_contents);

        Ok(())
    } else {
        Err(ConfigCliError::InvalidConfigName(dependency.0))
    }
}
