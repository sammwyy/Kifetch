use crate::{config::Config, constants::is_debug};
use std::{collections::HashMap, process::Command, time::Instant};

use bios_info::load_bios_info;
use cpu_info::load_cpu_info;
use disk_info::load_disk_info;
use env_info::load_env_info;
use gpu_info::load_gpu_info;
use kernel_info::load_kernel_info;
use memory_info::load_memory_info;
use net_info::load_net_info;
use os_info::load_os_info;
use packages_info::load_packages_info;
use screen_info::load_screen_info;
use uptime_info::load_uptime_info;

pub mod bios_info;
pub mod cpu_info;
pub mod disk_info;
pub mod env_info;
pub mod gpu_info;
pub mod kernel_info;
pub mod memory_info;
pub mod net_info;
pub mod os_info;
pub mod packages_info;
pub mod screen_info;
pub mod uptime_info;

pub struct SystemInfo {
    info: HashMap<String, String>,
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            info: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.info.get(key)
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.info.insert(key, value);
    }
}

// Resolve all modules
pub fn get_system_info(config: &Config) -> SystemInfo {
    let mut system_info = SystemInfo::new();
    let debug = is_debug();

    // Check if module is enabled to avoid unnecessary work
    for module in &config.modules.enabled {
        let start_time = if debug { Some(Instant::now()) } else { None };

        match module.as_str() {
            "os" => load_os_info(&mut system_info),
            "kernel" => load_kernel_info(&mut system_info),
            "uptime" => load_uptime_info(&mut system_info),
            "packages" => load_packages_info(&mut system_info),
            "env" => load_env_info(&mut system_info),
            "cpu" => load_cpu_info(&mut system_info),
            "gpu" => load_gpu_info(&mut system_info),
            "memory" => load_memory_info(&mut system_info),
            "screen" => load_screen_info(&mut system_info),
            "bios" => load_bios_info(&mut system_info),
            "disk" => load_disk_info(&mut system_info),
            "net" => load_net_info(&mut system_info),
            _ => {}
        }

        if debug {
            let end_time = Instant::now();
            let duration = end_time.duration_since(start_time.unwrap());
            println!("Loaded module {} in {}ms", module, duration.as_millis());
        }
    }

    // Load custom modules
    for (key, cmd) in &config.modules.custom {
        if let Ok(output) = Command::new("sh").arg("-c").arg(cmd).output() {
            if output.status.success() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                system_info.insert(format!("custom_{}", key), value);
            }
        }
    }

    system_info
}
