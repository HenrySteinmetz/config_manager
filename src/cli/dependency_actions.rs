use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum DependencyActions {
    Remove {
        dependency_name: String,
    },
    Add {
        dependency_name: String,
        /// Links dependency to the config
        config_name: Option<String>,
    },
    List {
        /// Lists only the dependencies of the provided config
        config_name: String,
    },
}
