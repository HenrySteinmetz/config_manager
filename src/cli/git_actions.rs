use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum GitActions {
    SetUrl { url: String },
    InstallTheme { url: String },
    Push { commit_message: Option<String> },
    Pull,
}
