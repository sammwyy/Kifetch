use std::{env, path::Path};

pub const INLINE_CONFIG_FILE: &str = "kifetch.toml";
pub const CONFIG_DIR: &str = ".config/kifetch";

pub fn get_config_path() -> String {
    // Get the current user's home directory
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    // Check if the config file exists
    if Path::new(INLINE_CONFIG_FILE).exists() {
        INLINE_CONFIG_FILE.to_string()
    } else {
        format!("{}/{}/config.toml", home_dir, CONFIG_DIR)
    }
}

pub fn get_config_dir() -> String {
    // Get the current user's home directory
    let home_dir = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    // Check if the config file exists
    if Path::new(INLINE_CONFIG_FILE).exists() {
        ".".to_string()
    } else {
        format!("{}/{}/", home_dir, CONFIG_DIR)
    }
}

pub fn is_debug() -> bool {
    env::var("KIFETCH_DEBUG").is_ok()
}
