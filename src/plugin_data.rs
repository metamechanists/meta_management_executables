use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;
use serde_json::from_str;

use crate::messages;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PluginData {
    Internal { servers: Vec<String> },
    Script { servers: Vec<String> },
    Manual { servers: Vec<String>, link: String },
    Spiget { servers: Vec<String>, id: i32 },
}

impl PluginData {
    pub fn load() -> HashMap<String, PluginData> {
        let file = File::open("plugin_data.json");
        let Ok(mut file) = file else {
            panic!("{}", messages::could_not_find_file("plugin_data.json"));
        };
    
        let mut json_string = String::new();
        if let Err(error) = file.read_to_string(&mut json_string) {
            panic!("{}", messages::error_reading_file("plugin_data.json", error));
        }
    
        match from_str(json_string.as_str()) {
            Err(error) => panic!("{}", messages::failed_to_parse_json(error)),
            Ok(plugin_data) => plugin_data,
        }
    }

    pub fn get_servers(&self) -> Vec<String> {
        match self {
            PluginData::Internal { servers } => servers.clone(),
            PluginData::Script { servers } => servers.clone(),
            PluginData::Manual { servers, link: _ } => servers.clone(),
            PluginData::Spiget { servers, id: _ } => servers.clone(),
        }
    }
}