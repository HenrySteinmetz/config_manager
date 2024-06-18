use super::{Dependency, DependencyFile, DependencyWrapper};
use crate::error::ConfigCliError;
use crate::utils::{get_base_dir, ConfigResult};
use crate::{try_read_and_parse, try_write_file};

pub fn add_dependency(
    theme: String,
    config: Option<String>,
    dependency: String,
) -> ConfigResult<()> {
    let path = get_base_dir()? + &theme + "/dependencies.toml";
    let mut file_contents = try_read_and_parse!(path.clone(), DependencyFile);
    let dependencies = &mut file_contents.globals;
    let config_dependencies: Vec<DependencyWrapper> = file_contents
        .config_bounds
        .iter()
        .map(|x| Into::<DependencyWrapper>::into(x))
        .collect();

    let dependency = Dependency(dependency);

    if config.is_some()
        && config_dependencies
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<String>>()
            .as_slice()
            .contains(config.as_ref().unwrap())
    {
        dependencies.extend(
            config_dependencies
                .into_iter()
                .map(|x| Into::<Dependency>::into(x))
                .collect::<Vec<Dependency>>(),
        );
    }

    if dependencies.contains(&dependency) {
        return Err(ConfigCliError::InvalidDependencyName(dependency.0));
    }

    if config.is_some() {
        file_contents
            .config_bounds
            .push((config.unwrap(), dependency));
    } else {
        file_contents.globals.push(dependency);
    }

    try_write_file!(path, &file_contents);
    Ok(())
}
