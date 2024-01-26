use std::{collections::HashMap, process::Command};

use crate::{messages, metadata::MetaData, plugin_data::PluginData};

pub fn update_all(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData) {
    for data in plugin_data {
        update_plugin(plugin_data, metadata, data.0);
    }
    update_paper(metadata);
    update_waterfall(metadata);
}

pub fn update_plugin(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData, plugin: &String) {
    let data = plugin_data.get(plugin).unwrap_or_else(|| panic!("{}", messages::could_not_find_plugin(plugin).as_str()));
    match data {
        PluginData::Internal => (),
        PluginData::Script => update_script(metadata, plugin),
        PluginData::Blob => update_blob(metadata, plugin),
        PluginData::Manual { link } => update_manual(plugin, link),
        PluginData::Spiget { id } => update_spiget(metadata, plugin, *id),
    }
}

pub fn update_paper(metadata: &MetaData) {
    let script = metadata.get_scripts_directory() + "/paper.sh";
    let output = Command::new("sh")
        .arg(script.as_str())
        .arg(metadata.get_paper_version())
        .arg(metadata.get_executables_directory())
        .output();
    match output {
        Err(error) => println!("{}", messages::paper_failed(error)),
        Ok(_) => println!("{}", messages::updated_paper()),
    }
}

pub fn update_waterfall(metadata: &MetaData) {
    let script = metadata.get_scripts_directory() + "/waterfall.sh";
    let output = Command::new("sh")
        .arg(script.as_str())
        .arg(metadata.get_waterfall_version())
        .arg(metadata.get_executables_directory())
        .output();
    match output {
        Err(error) => println!("{}", messages::waterfall_failed(error)),
        Ok(_) => println!("{}", messages::updated_waterfall()),
    }
}


fn update_script(metadata: &MetaData, plugin: &String) {
    let script = metadata.get_scripts_directory() + "/" + plugin + ".sh";
    let output = Command::new("sh")
        .arg(script.as_str())
        .arg(metadata.get_executables_directory())
        .output();
    match output {
        Err(error) => println!("{}", messages::script_failed(&script, error)),
        Ok(_) => println!("{}", messages::updated_script(plugin)),
    }
}

fn update_blob(metadata: &MetaData, plugin: &String) {
    let script = metadata.get_scripts_directory() + "/blob.sh";
    let output = Command::new("sh")
        .arg(script.as_str())
        .arg(plugin)
        .arg(metadata.get_executables_directory())
        .output();
    match output {
        Err(error) => println!("{}", messages::script_failed(&script, error)),
        Ok(_) => println!("{}", messages::updated_script(plugin)),
    }
}

fn update_manual(plugin: &String, link: &String) {
    println!("{}", messages::updated_manual(plugin, link));
}

fn update_spiget(metadata: &MetaData, plugin: &String, id: i32) {
    let script = metadata.get_scripts_directory() + "/spiget.sh";
    let output = Command::new("sh")
        .arg(script.as_str())
        .arg(id.to_string())
        .arg(metadata.get_executables_directory())
        .arg(plugin)
        .output();
    match output {
        Err(error) => println!("{}", messages::spiget_failed(plugin, error)),
        Ok(_) => println!("{}", messages::updated_spiget(plugin)),
    }
}