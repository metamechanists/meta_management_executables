use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;
use serde_json::from_str;

use crate::messages;

#[derive(Deserialize)]
pub struct MetaData {
    paper_version: String,
    waterfall_version: String,
    scripts: String,
    executables: String,
    deploy_from: String,
    servers: HashMap<String, String>,
}

impl MetaData {
    pub fn load() -> Self {
        let file = File::open("metadata.json");
        let Ok(mut file) = file else {
            panic!("{}", messages::could_not_find_file("metadata.json"));
        };

        let mut json_string = String::new();
        if let Err(error) = file.read_to_string(&mut json_string) {
            panic!("{}", messages::error_reading_file("metadata.json", error));
        }

        match from_str(json_string.as_str()) {
            Err(error) => panic!("{}", messages::failed_to_parse_json(error)),
            Ok(paths) => paths,
        }
    }

    pub fn get_server_directory(&self, server: &String) -> String {
        self.servers.get(server)
            .unwrap_or_else(|| panic!("{}", messages::server_does_not_exist(server).as_str()))
            .to_string()
    }

    pub fn get_deploy_from_directory(&self) -> String {
        self.get_server_directory(&self.deploy_from)
    }

    pub fn get_scripts_directory(&self) -> String {
        self.scripts.clone()
    }

    pub fn get_executables_directory(&self) -> String {
        self.executables.clone()
    }

    pub fn get_paper_version(&self) -> String {
        self.paper_version.clone()
    }

    pub fn get_waterfall_version(&self) -> String{
        self.waterfall_version.clone()
    }

    pub fn get_servers(&self) -> Vec<String> {
        self.servers.keys().cloned().collect()
    }
}