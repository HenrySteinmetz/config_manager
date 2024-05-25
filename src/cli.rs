use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct ConfigCli {
    #[command(subcommand)]
    pub command: ConfigSubCommands,
}

#[derive(Subcommand, Clone)]
pub enum ConfigSubCommands {
    Dependency {
        #[command(subcommand)]
        action: DependencyActions,
    },

    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },

    Theme {
        #[command(subcommand)]
        action: ThemeActions,
    },
}

#[derive(Subcommand, Clone)]
pub enum DependencyActions {
    Remove {
        theme_name: String,
        dependency_name: String,
    },
    Add {
        theme_name: String,
        dependency_name: String,
        /// Links dependency to the config
        config_name: Option<String>,
    },
    List {
        theme_name: String,
        /// Lists only the dependencies of the provided config
        config_name: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum ConfigActions {
    /// Moves the original config back if no other theme uses a config with the same name
    Remove {
        theme_name: String,
        config_name: String,
    },
    /// Moves the original config file while replacing it with a symlink
    Add {
        theme_name: String,
        config_name: String,
        file: PathBuf,
        device_name: Option<String>,
    },
    List {
        theme_name: String,
        device_name: Option<String>,
    },
}

#[derive(Subcommand, Clone)]
pub enum ThemeActions {
    Remove {
        name: String,
    },
    Create {
        name: String,
        /// Creates a new Theme by copying the provided theme as a base
        base: Option<String>,
    },
    /// Links all the used config files to the according folders
    Use {
        name: String,
    },
    List,
}
