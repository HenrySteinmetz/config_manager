use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum DeviceActions {
    Remove { name: String },
    Add { name: String },
    Use { name: String },
    List,
}
