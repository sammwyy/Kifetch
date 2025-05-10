use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub colors: HashMap<String, String>,
    pub modules: ModulesConfig,
    pub layout: LayoutConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub logo: String,
    pub logo_path: Option<String>,
    pub separator: String,
    pub padding: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModulesConfig {
    pub enabled: Vec<String>,
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LayoutConfig {
    pub lines: Vec<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        if !Path::new(path).exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Config {
            general: GeneralConfig {
                logo: "linux".to_string(),
                logo_path: None,
                separator: ": ".to_string(),
                padding: 2,
            },
            colors: [
                ("color_1".to_string(), "bright_red".to_string()),
                ("color_2".to_string(), "bright_green".to_string()),
                ("color_3".to_string(), "bright_yellow".to_string()),
                ("color_4".to_string(), "bright_blue".to_string()),
                ("color_5".to_string(), "bright_magenta".to_string()),
                ("color_6".to_string(), "bright_cyan".to_string()),
                ("color_reset".to_string(), "reset".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
            modules: ModulesConfig {
                enabled: vec![
                    "os".to_string(),
                    "kernel".to_string(),
                    "uptime".to_string(),
                    "packages".to_string(),
                    "shell".to_string(),
                    "cpu".to_string(),
                    "gpu".to_string(),
                    "memory".to_string(),
                ],
                custom: HashMap::new(),
            },
            layout: LayoutConfig {
                lines: vec![
                    "{color_1}{os_name}{color_reset}".to_string(),
                    "{color_2}Kernel{color_reset}{separator}{kernel_version}".to_string(),
                    "{color_3}Host{color_reset}{separator}{os_hostname}".to_string(),
                    "{color_4}Uptime{color_reset}{separator}{uptime}".to_string(),
                    "{color_5}CPU{color_reset}{separator}{cpu_model}".to_string(),
                    "{color_6}Memory{color_reset}{separator}{memory_used}/{memory_total}"
                        .to_string(),
                ],
            },
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string_pretty(self)?;

        // Ensure parent directory exists
        if let Some(parent) = Path::new(path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        fs::write(path, content)?;
        Ok(())
    }
}
