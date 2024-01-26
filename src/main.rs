use clap::{Parser, Subcommand};
use deploy::{deploy_all, deploy_plugin};
use metadata::MetaData;
use plugin_data::PluginData;
use update::{update_all, update_paper, update_plugin, update_waterfall};

mod deploy;
mod messages;
mod metadata;
mod plugin_data;
mod update;

#[derive(Clone, Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    Update { plugin: String },
    Deploy { plugin: String }
}

fn main() {
    let cli = Cli::parse();
    let plugin_data = PluginData::load();
    let metadata = MetaData::load();
    match cli.command {
        Commands::Update { plugin } => {
            if plugin.to_lowercase().as_str() == "all" {
                update_all(plugin_data, metadata);
            } else if plugin.to_lowercase().as_str() == "paper" {
                update_paper(&metadata);
            } else if plugin.to_lowercase().as_str() == "waterfall" {
                update_waterfall(&metadata);
            } else {
                update_plugin(&plugin_data, &metadata, &plugin);
            }
        },
        Commands::Deploy { plugin } => {
            if plugin.to_lowercase().as_str() == "all" {
                deploy_all(plugin_data, metadata);
            } else {
                deploy_plugin(&plugin_data, &metadata, &plugin);
            }
        },  
    }
}
