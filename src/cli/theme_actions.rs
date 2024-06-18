use clap::Subcommand;

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
        #[arg(short, long, default_value_t = false)]
        force: bool,
        device: Option<String>,
    },
    List,
}
