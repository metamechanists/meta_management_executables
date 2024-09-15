use std::{collections::HashMap, process::Command};

use crate::{messages, metadata::MetaData, plugin_data::PluginData};

pub fn deploy_all(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData) {
    for data in plugin_data {
        if let PluginData::Internal = data.1 {
            deploy_plugin(plugin_data, metadata, data.0);
        }
    }
}

pub fn deploy_plugin(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData, plugin: &String) {
    let data = plugin_data.get(plugin).unwrap_or_else(|| panic!("{}", messages::could_not_find_plugin(plugin)));
    let PluginData::Internal = data else {
        panic!("{}", messages::plugin_is_not_internal(plugin).as_str());
    };
    
    let jar_file_name = plugin.clone() + ".jar";
    let from = metadata.get_deploy_from_directory() + "/plugins/" + jar_file_name.as_str();
    let to = metadata.get_executables_directory() + "/" + jar_file_name.as_str();
    if let Err(error) = Command::new("cp")
            .arg(&from)
            .arg(&to)
            .output() {
        panic!("{}", messages::failed_to_copy_jar(&from, &to, error));
    }

    println!("{}", messages::deployed(plugin));
}