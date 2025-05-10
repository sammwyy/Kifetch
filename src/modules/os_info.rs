use std::env;

use super::SystemInfo;

struct OsInfo {
    name: String,
    version: String,
    hostname: String,
}

impl Default for OsInfo {
    fn default() -> Self {
        OsInfo {
            name: format!("{} {}", env::consts::OS, env::consts::ARCH),
            version: "Unknown".to_string(),
            hostname: "Unknown".to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_os() -> OsInfo {
    use std::fs;

    let mut os = OsInfo::default();

    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        for line in os_release.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let value = line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
                os.name = value;
                break;
            }
        }
    }

    if let Ok(hostname) = fs::read_to_string("/etc/hostname") {
        os.hostname = hostname.trim().to_string();
    }

    return os;
}

#[cfg(target_os = "windows")]
fn get_os() -> OsInfo {
    use std::{ffi::OsString, os::windows::ffi::OsStringExt};
    use windows::{
        core::PWSTR,
        Win32::System::SystemInformation::{
            ComputerNameDnsHostname, GetComputerNameExW, GetVersionExW, OSVERSIONINFOEXW,
        },
    };
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let mut os = OsInfo::default();

    // Get hostname info
    let mut computer_name: Vec<u16> = vec![0; 256];
    let mut size = computer_name.len() as u32;

    unsafe {
        let result = GetComputerNameExW(
            ComputerNameDnsHostname,
            Some(PWSTR(computer_name.as_mut_ptr())),
            &mut size,
        );

        if !result.is_ok() {
            println!("Failed to get computer name: {}", result.unwrap_err());
            return os;
        }
    }

    let computer_name = OsString::from_wide(&computer_name)
        .to_string_lossy()
        .to_string();
    os.hostname = computer_name;

    // Get OS version and name
    let mut version_info = OSVERSIONINFOEXW {
        dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOEXW>() as u32,
        ..Default::default()
    };

    unsafe {
        let _ = GetVersionExW(&mut version_info as *mut _ as *mut _);
    }

    let mut version = String::new();
    version.push_str(&format!(
        "{}.{}.{}",
        version_info.dwMajorVersion, version_info.dwMinorVersion, version_info.dwBuildNumber
    ));

    // Try to get edition info from registry
    let hkey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";

    match hkey.open_subkey(subkey) {
        Ok(regkey) => {
            if let Ok(value) = regkey.get_value::<String, _>("ProductName") {
                os.name = value; // Use the ProductName directly from the registry
            }
        }
        Err(_) => {
            os.name = "Unknown Edition".to_string();
        }
    }

    os.version = version;

    os
}

pub fn load_os_info(info: &mut SystemInfo) {
    let os = get_os();

    info.insert("os_name".to_string(), os.name);
    info.insert("os_version".to_string(), os.version);
    info.insert("os_hostname".to_string(), os.hostname);
}
