use std::error::Error;

use crate::config::Config;
use crate::modules::SystemInfo;

// ANSI color map
const COLOR_MAP: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "black" => "\x1b[30m",
    "red" => "\x1b[31m",
    "green" => "\x1b[32m",
    "yellow" => "\x1b[33m",
    "blue" => "\x1b[34m",
    "magenta" => "\x1b[35m",
    "cyan" => "\x1b[36m",
    "white" => "\x1b[37m",
    "bright_black" => "\x1b[90m",
    "bright_red" => "\x1b[91m",
    "bright_green" => "\x1b[92m",
    "bright_yellow" => "\x1b[93m",
    "bright_blue" => "\x1b[94m",
    "bright_magenta" => "\x1b[95m",
    "bright_cyan" => "\x1b[96m",
    "bright_white" => "\x1b[97m",
    "reset" => "\x1b[0m",
    "bold" => "\x1b[1m",
    "dim" => "\x1b[2m",
    "italic" => "\x1b[3m",
    "underline" => "\x1b[4m",
    "blink" => "\x1b[5m",
    "reverse" => "\x1b[7m",
    "hidden" => "\x1b[8m",
};

// Function to render the complete output
pub fn render_output(
    logo: &[String],
    config: &Config,
    system_info: &SystemInfo,
) -> Result<(), Box<dyn Error>> {
    let mut logo_iter = logo.iter();
    let padding = " ".repeat(config.general.padding);

    // Process each line of the layout
    for (_, line_template) in config.layout.lines.iter().enumerate() {
        // Get the corresponding logo line, or an empty line if there are no more
        let logo_line = if let Some(line) = logo_iter.next() {
            line
        } else {
            &" ".repeat(logo[0].len())
        };

        // Apply variables to the template
        let line = render_template(line_template, config, system_info);

        // Print the line with the logo
        println!("{}{}{}", logo_line, padding, line);
    }

    // Print remaining logo lines, if any
    for logo_line in logo_iter {
        println!("{}", logo_line);
    }

    Ok(())
}

// Function to render a template with variables
fn render_template(template: &str, config: &Config, system_info: &SystemInfo) -> String {
    let mut result = template.to_string();

    // Replace system variables
    let system_vars_regex = regex::Regex::new(r"\{([a-zA-Z0-9_]+)\}").unwrap();
    result = system_vars_regex
        .replace_all(&result, |caps: &regex::Captures| {
            let var_name = &caps[1];

            // Check if it's a color variable
            if let Some(color_value) = config.colors.get(var_name) {
                if let Some(ansi_code) = COLOR_MAP.get(color_value.as_str()) {
                    return ansi_code.to_string();
                }
            }

            // Check if it's a system variable
            if let Some(value) = system_info.get(var_name) {
                return value.clone();
            }

            // Check if it's a separator variable
            if var_name == "separator" {
                return config.general.separator.clone();
            }

            // If not found, leave the variable unchanged
            format!("{{{}}}", var_name)
        })
        .to_string();

    result
}
