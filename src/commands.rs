use std::collections::HashMap;

use crate::{metadata::MetaData, plugin_data::PluginData};

use self::{deploy::{deploy_all, deploy_plugin}, link::{link_paper, link_plugin, link_waterfall}, list::{list_paper, list_plugin, list_waterfall}, unlink::{unlink_paper, unlink_plugin, unlink_waterfall}, update::{update_all, update_paper, update_plugin, update_waterfall}};

mod deploy;
mod integrity;
mod link;
mod list;
mod unlink;
mod update;
mod verify;

pub fn update(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData, plugin: String) {
    match plugin.to_lowercase().as_str() {
        "all" => update_all(plugin_data, metadata),
        "paper" => update_paper(metadata),
        "waterfall" => update_waterfall(metadata),
        _ => update_plugin(plugin_data, metadata, &plugin),
    }
    integrity::integrity(metadata);
    verify::verify(metadata, plugin_data);
}

pub fn deploy(plugin_data: &HashMap<String, PluginData>, metadata: &MetaData, plugin: String) {
    match plugin.to_lowercase().as_str() {
        "all" => deploy_all(plugin_data, metadata),
        _ => deploy_plugin(plugin_data, metadata, &plugin),
    }
    integrity::integrity(metadata);
    verify::verify(metadata, plugin_data);
}

pub fn list(metadata: &MetaData, plugin: String) {
    match plugin.to_lowercase().as_str() {
        "paper" => list_paper(metadata),
        "waterfall" => list_waterfall(metadata),
        _ => list_plugin(metadata, plugin),
    }
}

pub fn link(metadata: &MetaData, plugin_data: &HashMap<String, PluginData>, plugin: String, server: String) {
    match plugin.to_lowercase().as_str() {
        "paper" => link_paper(metadata, server),
        "waterfall" => link_waterfall(metadata, server),
        _ => link_plugin(metadata, plugin, server),
    }
    integrity::integrity(metadata);
    verify::verify(metadata, plugin_data);
}

pub fn unlink(metadata: &MetaData, plugin: String, server: String) {
    match plugin.to_lowercase().as_str() {
        "paper" => unlink_paper(metadata, server),
        "waterfall" => unlink_waterfall(metadata, server),
        _ => unlink_plugin(metadata, plugin, server),
    }
}

pub fn verify(metadata: &MetaData, plugin_data: &HashMap<String, PluginData>) {
        verify::verify(metadata, plugin_data)
}

pub fn integrity(metadata: &MetaData) {
    integrity::integrity(metadata);
}