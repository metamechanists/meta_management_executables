use clap::{Parser, Subcommand};
use commands::{deploy, link, list, unlink, update};
use metadata::MetaData;
use plugin_data::PluginData;

mod commands;
mod messages;
mod metadata;
mod plugin_data;

#[derive(Clone, Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    /// Update a specific plugin. 'all', 'paper', and 'waterfall' are valid.
    Update { plugin: String },
    /// Deploy a specific plugin from the dev server. 'all' is valid.
    Deploy { plugin: String },
    /// List servers a plugin is linked to
    List { plugin: String },
    /// Link a plugin to a server. 'paper' and 'waterfall' are valid.
    Link { plugin: String, server: String },
    /// Unlink a plugin from a server. 'paper' and 'waterfall' are valid.
    Unlink { plugin: String, server: String },
}

fn main() {
    let cli = Cli::parse();
    let plugin_data = PluginData::load();
    let metadata = MetaData::load();
    match cli.command {
        Commands::Update { plugin } => update(&plugin_data, &metadata, plugin),
        Commands::Deploy { plugin } => deploy(&plugin_data, &metadata, plugin),
        Commands::List { plugin } => list(&metadata, plugin),
        Commands::Link { plugin, server } => link(&metadata, plugin, server),
        Commands::Unlink { plugin, server } => unlink(&metadata, plugin, server),
    }
}
