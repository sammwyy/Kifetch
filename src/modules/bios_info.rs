use super::SystemInfo;

struct BiosInfo {
    vendor: String,
    version: String,
    motherboard: String,
}

impl Default for BiosInfo {
    fn default() -> Self {
        BiosInfo {
            vendor: "unknown".to_string(),
            version: "unknown".to_string(),
            motherboard: "unknown".to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_bios_info() -> BiosInfo {
    use std::fs;

    let mut bios = BiosInfo::default();

    fn read_trimmed(path: &str) -> String {
        fs::read_to_string(path)
            .unwrap_or_default()
            .trim()
            .to_string()
    }

    bios.vendor = read_trimmed("/sys/class/dmi/id/bios_vendor");
    bios.version = read_trimmed("/sys/class/dmi/id/bios_version");
    bios.motherboard = read_trimmed("/sys/class/dmi/id/board_name");

    bios
}

#[cfg(target_os = "windows")]
fn get_bios_info() -> BiosInfo {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let bios_key = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\BIOS");
    let mut bios = BiosInfo::default();

    if let Ok(key) = bios_key {
        bios.vendor = key.get_value("BIOSVendor").unwrap_or("Unknown".to_string());
        bios.version = key
            .get_value("BIOSVersion")
            .unwrap_or("Unknown".to_string());
        bios.motherboard = key
            .get_value("BaseBoardProduct")
            .unwrap_or("Unknown".to_string());
    }

    bios
}

pub fn load_bios_info(info: &mut SystemInfo) {
    let bios = get_bios_info();
    info.insert("bios_vendor".to_string(), bios.vendor);
    info.insert("bios_version".to_string(), bios.version);
    info.insert("bios_motherboard".to_string(), bios.motherboard);
}
