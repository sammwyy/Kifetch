use std::error::Error;
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::constants::get_config_dir;

// Normalize logo spacing
fn normalize_logo(logo: &str) -> Vec<String> {
    let lines: Vec<&str> = logo.lines().collect();
    let mut result = Vec::new();

    // Skip empty spaces
    let mut start = 0;
    let mut end = lines.len();

    while start < end && lines[start].trim().is_empty() {
        start += 1;
    }

    while end > start && lines[end - 1].trim().is_empty() {
        end -= 1;
    }

    // Find the maximum width
    let max_width = lines[start..end]
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    // Normalize each line
    for i in start..end {
        let line = lines[i];
        // Add spaces to the right of the line
        let padding = " ".repeat(max_width - line.len());
        result.push(format!("{}{}", line, padding));
    }

    result
}

// Find logo content
pub fn load_logo(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let logo_name = &config.general.logo;

    // Check if the logo is a custom path
    if let Some(ref custom_path) = config.general.logo_path {
        if Path::new(custom_path).exists() {
            let logo_content = fs::read_to_string(custom_path)?;
            return Ok(normalize_logo(&logo_content));
        }
    }

    // Check on current directory
    let current_path = format!("logos/{}.txt", logo_name);
    if Path::new(&current_path).exists() {
        let logo_content = fs::read_to_string(&current_path)?;
        return Ok(normalize_logo(&logo_content));
    }

    // Check on the install path
    let install_path = get_config_dir();
    let logos_path = format!("{}/logos/{}.txt", install_path, logo_name);

    if Path::new(&logos_path).exists() {
        let logo_content = fs::read_to_string(&logos_path)?;
        return Ok(normalize_logo(&logo_content));
    }

    // Use default logo
    Ok(normalize_logo(DEFAULT_LOGO))
}

const DEFAULT_LOGO: &str = r#"
    .---.
   /     \
   \.@-@./
   /`\_/`\
  //  _  \\
 | \     )|_
/`\_`>  <_/ \
\__/'---'\__/
"#;
