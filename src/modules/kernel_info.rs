use super::SystemInfo;

struct KernelInfo {
    version: String,
}

impl Default for KernelInfo {
    fn default() -> Self {
        KernelInfo {
            version: "Unknown".to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_kernel() -> KernelInfo {
    use std::process::Command;

    let mut info = KernelInfo::default();
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        info.version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
    return info;
}

#[cfg(target_os = "windows")]
fn get_kernel() -> KernelInfo {
    use windows::Win32::System::SystemInformation::OSVERSIONINFOW;
    let mut info = KernelInfo::default();

    extern "system" {
        fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOW) -> i32;
    }

    let mut version_info = OSVERSIONINFOW {
        dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOW>() as u32,
        ..Default::default()
    };

    unsafe {
        if RtlGetVersion(&mut version_info) == 0 {
            info.version = format!(
                "{}.{}.{}",
                version_info.dwMajorVersion,
                version_info.dwMinorVersion,
                version_info.dwBuildNumber
            );
        }
    }

    info
}

pub fn load_kernel_info(info: &mut SystemInfo) {
    let kernel = get_kernel();
    info.insert("kernel_version".to_string(), kernel.version);
}
