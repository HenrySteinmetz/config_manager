use clap::{Parser, Subcommand};

pub mod config_actions;
pub mod dependency_actions;
pub mod device_actions;
pub mod git_actions;
pub mod theme_actions;

pub use config_actions::ConfigActions;
pub use dependency_actions::DependencyActions;
pub use device_actions::DeviceActions;
pub use git_actions::GitActions;
pub use theme_actions::ThemeActions;

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

    Device {
        #[command(subcommand)]
        action: DeviceActions,
    },

    Theme {
        #[command(subcommand)]
        action: ThemeActions,
    },

    Git {
        #[command(subcommand)]
        action: GitActions,
    },
}
