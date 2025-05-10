use std::collections::HashMap;

use super::SystemInfo;

struct PackagesInfo {
    count: usize,
    total_count: usize,
    manager: String,
    packages: HashMap<String, usize>,
}

impl Default for PackagesInfo {
    fn default() -> Self {
        Self {
            count: 0,
            total_count: 0,
            manager: "Unknown".to_string(),
            packages: HashMap::new(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_packages() -> PackagesInfo {
    use std::process::Command;

    let mut info = PackagesInfo::default();

    // Count packages on different package managers
    let package_managers = [
        ("dpkg", "dpkg-query -f '${binary:Package}\n' -W | wc -l"),
        ("rpm", "rpm -qa | wc -l"),
        ("pacman", "pacman -Q | wc -l"),
    ];

    let mut is_primary = true;

    for (manager, cmd) in &package_managers {
        if let Ok(output) = Command::new("sh").arg("-c").arg(cmd).output() {
            if output.status.success() {
                let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let count = count_str.parse::<usize>().unwrap();

                info.packages.insert(manager.to_string(), count);
                info.total_count += count;

                if is_primary {
                    info.count = count;
                    info.manager = manager.to_string();
                    is_primary = false;
                }
            }
        }
    }

    return info;
}

#[cfg(target_os = "windows")]
fn get_packages() -> PackagesInfo {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let mut info = PackagesInfo::default();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let uninstall_paths = [
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    ];

    for path in uninstall_paths {
        if let Ok(key) = hklm.open_subkey(path) {
            if let Ok(subkeys) = key.enum_keys().collect::<Result<Vec<_>, _>>() {
                info.count += subkeys.len();
            }
        }
    }

    info.manager = "registry".to_string();
    info.total_count = info.count;
    return info;
}

pub fn load_packages_info(info: &mut SystemInfo) {
    let packages = get_packages();

    info.insert(
        "packages_total".to_string(),
        packages.total_count.to_string(),
    );

    info.insert("package_manager".to_string(), packages.manager.to_string());

    info.insert("packages".to_string(), packages.count.to_string());

    for (manager, count) in packages.packages.iter() {
        info.insert(format!("packages_{}", manager), count.to_string());
    }
}
