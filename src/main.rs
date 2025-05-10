use std::error::Error;

mod config;
mod constants;
mod logo;
mod modules;
mod render;
mod utils;

use config::Config;
use constants::get_config_path;
use modules::get_system_info;
use render::render_output;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the config path
    let config_path = get_config_path();

    // Load the config
    let config = match Config::load(&config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            Config::default()
        }
    };

    // Load logo
    let logo_content = logo::load_logo(&config)?;

    // Get system info
    let system_info = get_system_info(&config);

    // Render the output
    render_output(&logo_content, &config, &system_info)?;

    Ok(())
}
