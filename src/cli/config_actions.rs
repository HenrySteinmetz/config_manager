use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Clone)]
pub enum ConfigActions {
    /// Moves the original config back if no other theme uses a config with the same name
    Remove {
        config_name: String,
    },
    /// Moves the original config file while replacing it with a symlink
    Add {
        config_name: String,
        file: PathBuf,
        device_name: Option<String>,
    },
    List {
        device_name: Option<String>,
    },
}
