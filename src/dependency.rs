use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::error::ConfigCliError;
use crate::utils::ConfigResult;

pub mod add;
pub mod list;
pub mod remove;

pub use add::*;
pub use list::*;
pub use remove::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dependency(String);

impl Dependency {
    pub fn install(&self) -> ConfigResult<()> {
        let output = match Command::new("yay").arg("-Q").arg(self.0.clone()).output() {
            Ok(cli) => cli,
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        };
        if output.status.code() == Some(0) {
            return Ok(());
        }
        let install_output = match Command::new("yay").arg("-S").arg(self.0.clone()).output() {
            Ok(cli) => cli,
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        };

        match install_output.status.code() {
            Some(0) => Ok(()),
            _ => Err(ConfigCliError::GitCommandError(
                install_output.status.to_string(),
            )),
        }
    }
}

impl TryFrom<String> for Dependency {
    type Error = ConfigCliError;
    fn try_from(value: String) -> ConfigResult<Self> {
        let output = match Command::new("yay").arg("-Ssq").arg(value.clone()).output() {
            Ok(cli) => cli.stdout,
            Err(err) => return Err(ConfigCliError::ShellInitError(err)),
        };

        let results: Vec<String> = match std::str::from_utf8(&output) {
            Ok(s) => s.to_owned().lines().map(|x| x.to_owned()).collect(),
            Err(err) => return Err(ConfigCliError::StringConversionError(err)),
        };

        if results.contains(&value) {
            Ok(Dependency(value))
        } else {
            Err(ConfigCliError::NoPackageWithName(value))
        }
    }
}

impl Into<Dependency> for (String, Dependency) {
    fn into(self) -> Dependency {
        self.1
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct DependencyFile {
    pub globals: Vec<Dependency>,
    pub config_bounds: Vec<(String, Dependency)>,
}

struct DependencyWrapper(String, Dependency);

impl FromIterator<DependencyWrapper> for Vec<Dependency> {
    fn from_iter<T: IntoIterator<Item = DependencyWrapper>>(iter: T) -> Self {
        iter.into_iter().map(|x| x.1).collect()
    }
}

impl Into<Dependency> for DependencyWrapper {
    fn into(self) -> Dependency {
        self.1
    }
}

impl Into<DependencyWrapper> for &(String, Dependency) {
    fn into(self) -> DependencyWrapper {
        DependencyWrapper(self.0.clone(), self.1.clone())
    }
}

impl Into<DependencyWrapper> for (String, Dependency) {
    fn into(self) -> DependencyWrapper {
        DependencyWrapper(self.0, self.1)
    }
}
