use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;
use serde_json::from_str;

use crate::messages;

const PLUGIN_DATA_PATH: &str = "/home/exec-manager/plugin_data.json";

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum PluginData {
    Internal,
    Script,
    Blob,
    Manual { link: String },
    Spiget { id: i32 },
}

impl PluginData {
    pub fn load() -> HashMap<String, PluginData> {
        let file = File::open(PLUGIN_DATA_PATH);
        let Ok(mut file) = file else {
            panic!("{}", messages::could_not_find_file(PLUGIN_DATA_PATH));
        };
    
        let mut json_string = String::new();
        if let Err(error) = file.read_to_string(&mut json_string) {
            panic!("{}", messages::error_reading_file(PLUGIN_DATA_PATH, error));
        }
    
        match from_str(json_string.as_str()) {
            Err(error) => panic!("{}", messages::failed_to_parse_json(error)),
            Ok(plugin_data) => plugin_data,
        }
    }
}