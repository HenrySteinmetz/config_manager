use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::dependency::Dependency;

pub mod add;
pub mod list;
pub mod remove;

pub use add::*;
pub use list::*;
pub use remove::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub name: String,
    pub dependencies: Vec<Dependency>,
    pub symlink: PathBuf,
    pub conf_location: PathBuf,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ConfigFile {
    pub globals: Vec<Config>,
    pub device_bounds: Vec<(String, Config)>,
}
