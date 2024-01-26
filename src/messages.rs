use std::io;

use colored::Colorize;

pub fn could_not_find_plugin(plugin: &String) -> String {
    format!("{}{}{}", "Could not find plugin '".bright_red(), plugin.bright_cyan(), "'".bright_red())
}

pub fn plugin_is_not_internal(plugin: &String) -> String {
    format!("{}{}{}", "Plugin '".bright_red(), plugin.bright_cyan(), "' is not internal".bright_red())
}

pub fn failed_to_copy_jar(from: &String, to: &String, error: io::Error) -> String {
    format!("{}{}{}{}{}{}", "Failed to copy jar from '".bright_red(), from.bright_cyan(), "' to '".bright_red(), to.bright_cyan(), "': ".bright_red(), error)
}

pub fn could_not_find_file(file: &str) -> String {
    format!("{}{}", "Could not find ".bright_red(), file.bright_cyan())
}

pub fn error_reading_file(file: &str, error: io::Error) -> String {
    format!("{}{}{}", file.bright_red(), " was found, but there was an error while reading it: ".bright_red(), error)
}

pub fn failed_to_parse_json(error: serde_json::Error) -> String {
    format!("{}{}", "Failed to parse JSON: ".bright_red(), error)
}

pub fn server_does_not_exist(server: &String) -> String {
    format!("{}{}{}", "Server '".bright_red(), server, "' does not exist".bright_red())
}

pub fn script_failed(script: &String, error: io::Error) -> String {
    format!("{}{}{}{}", "Failed to execute ".bright_red(), script.bright_cyan(), ": ".bright_red(), error)
}

pub fn spiget_failed(plugin: &String, error: io::Error) -> String {
    format!("{}{}{}{}", "Failed to spiget ".bright_red(), plugin.bright_cyan(), ": ".bright_red(), error)
}

pub fn paper_failed(error: io::Error) -> String {
    format!("{}{}", "Failed to fetch paper: ".bright_red(), error)
}

pub fn waterfall_failed(error: io::Error) -> String {
    format!("{}{}", "Failed to fetch waterfall: ".bright_red(), error)
}

pub fn link_failed(error: io::Error) -> String {
    format!("{}{}", "Failed to link: ".bright_red(), error)
}

pub fn unlink_failed(error: io::Error) -> String {
    format!("{}{}", "Failed to unlink: ".bright_red(), error)
}

pub fn deployed(plugin: &String) -> String {
    format!("{}{}", "Successfully deployed ".bright_green(), plugin.bright_cyan())
}

pub fn updated_spiget(plugin: &String) -> String {
    format!("{}{}", "Successfully updated (spiget) ".bright_green(), plugin.bright_cyan())
}

pub fn updated_script(plugin: &String) -> String {
    format!("{}{}", "Successfully updated (script) ".bright_green(), plugin.bright_cyan())
}

pub fn updated_paper() -> String {
    format!("{}", "Successfully updated paper".bright_green())
}

pub fn updated_waterfall() -> String {
    format!("{}", "Successfully updated waterfall".bright_green())
}

pub fn updated_manual(plugin: &String, link: &String) -> String {
    format!("{}{}{}{}", "No update method for ".bright_yellow(), plugin.bright_cyan(), "; manually update it at ".bright_yellow(), link.bright_blue().underline())
}

pub fn linked_list_item(server: String) -> String {
    format!("{}{}", "- ".bright_green(), server.bright_cyan())
}