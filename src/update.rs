use std::{collections::HashMap, process::Command};

use crate::{messages, metadata::MetaData, plugin_data::PluginData};

pub fn update_all(plugin_data: HashMap<String, PluginData>, metadata: MetaData) {
    for data in &plugin_data {
        update_plugin(&plugin_data, &metadata, data.0);
    }
    update_paper(&metadata);
    update_waterfall(&metadata);
}

pub fn update_plugin(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData, plugin: &String) {
    let data = plugin_data.get(plugin).expect(messages::could_not_find_plugin(plugin).as_str());
    match data {
        PluginData::Internal { servers: _ } => (),
        PluginData::Script { servers: _ } => update_script(metadata, plugin),
        PluginData::Manual { servers: _, link } => update_manual(plugin, &link),
        PluginData::Spiget { servers: _, id } => update_spiget(metadata, plugin, *id),
    }
}

pub fn update_paper(metadata: &MetaData) {
    let script = metadata.get_scripts_directory() + "/paper.sh";
    if let Err(error) = Command::new("sh")
        .arg(script.as_str())
        .arg(metadata.get_paper_version())
        .arg(metadata.get_executables_directory())
        .output() {
        println!("{}", messages::paper_failed(error));
    } else {
        println!("{}", messages::updated_paper());
    }
}

pub fn update_waterfall(metadata: &MetaData) {
    let script = metadata.get_scripts_directory() + "/waterfall.sh";
    if let Err(error) = Command::new("sh")
        .arg(script.as_str())
        .arg(metadata.get_waterfall_version())
        .arg(metadata.get_executables_directory())
        .output() {
        println!("{}", messages::waterfall_failed(error));
    } else {
        println!("{}", messages::updated_waterfall());
    }
}


fn update_script(metadata: &MetaData, plugin: &String) {
    let script = metadata.get_scripts_directory() + "/" + plugin + ".sh";
    if let Err(error) = Command::new("sh")
        .arg(script.as_str())
        .arg(metadata.get_executables_directory())
        .output() {
        println!("{}", messages::script_failed(&script, error));
    } else {
        println!("{}", messages::updated_script(plugin));
    }
}

fn update_manual(plugin: &String, link: &String) {
    println!("{}", messages::updated_manual(plugin, link));
}

fn update_spiget(metadata: &MetaData, plugin: &String, id: i32) {
    let script = metadata.get_scripts_directory() + "/spiget.sh";
    if let Err(error) = Command::new("sh")
        .arg(script.as_str())
        .arg(id.to_string())
        .arg(metadata.get_executables_directory())
        .arg(plugin)
        .output() {
        println!("{}", messages::spiget_failed(&plugin, error));
    } else {
        println!("{}", messages::updated_spiget(plugin));
    }
}