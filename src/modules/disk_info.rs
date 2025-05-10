use super::SystemInfo;
use crate::utils::insert_size;

/// Represents disk usage information with used, free and total space in bytes
struct DiskInfo {
    used: usize,  // Space used in bytes
    free: usize,  // Free space available in bytes
    total: usize, // Total disk capacity in bytes
}

impl Default for DiskInfo {
    fn default() -> DiskInfo {
        DiskInfo {
            used: 0,
            free: 0,
            total: 0,
        }
    }
}

#[cfg(target_os = "linux")]
fn get_disk() -> DiskInfo {
    use libc::statvfs;
    use std::ffi::CString;
    use std::fs;

    let mut total: u64 = 0;
    let mut free: u64 = 0;

    // Filesystem types to ignore (virtual/special filesystems)
    let fs_types_to_ignore = [
        "sysfs",
        "proc",
        "devpts",
        "tmpfs",
        "devtmpfs",
        "debugfs",
        "securityfs",
        "cgroup",
        "pstore",
        "autofs",
        "mqueue",
        "hugetlbfs",
        "fusectl",
        "fuse.gvfsd-fuse",
    ];

    // Read mounted filesystems from /proc/mounts
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                continue; // Skip invalid lines
            }

            let device = parts[0];
            let mount_point = parts[1];
            let fs_type = parts[2];

            // Skip virtual/special filesystems
            if fs_types_to_ignore.contains(&fs_type) || device.starts_with("none") {
                continue;
            }

            // Convert mount point to C string for statvfs call
            let c_path = match CString::new(mount_point) {
                Ok(c) => c,
                Err(_) => continue, // Skip if path conversion fails
            };

            // Get filesystem statistics
            let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
            let res = unsafe { statvfs(c_path.as_ptr(), &mut stat) };
            if res != 0 {
                continue; // Skip if statvfs fails
            }

            let block_size = stat.f_frsize as u64;

            // Only count physical devices (blocks > 0) and avoid duplicate counting
            // of the same device mounted at multiple points
            if stat.f_blocks > 0 {
                total += stat.f_blocks as u64 * block_size;
                free += stat.f_bavail as u64 * block_size;
            }
        }
    }

    // Calculate used space (handle potential overflow)
    let used = if total > free { total - free } else { 0 };

    DiskInfo {
        used: used as usize,
        free: free as usize,
        total: total as usize,
    }
}

#[cfg(target_os = "windows")]
fn get_disk() -> DiskInfo {
    use windows::core::PWSTR as CorePWSTR;
    use windows::Win32::Storage::FileSystem::{GetDiskFreeSpaceExW, GetLogicalDriveStringsW};

    let mut total: u64 = 0;
    let mut free: u64 = 0;

    // Get list of available logical drives
    let mut drive_strings: Vec<u16> = vec![0; 260]; // MAX_PATH length buffer
    let len = unsafe { GetLogicalDriveStringsW(Some(&mut drive_strings)) };

    if len > 0 {
        // Process each logical drive
        let mut drive_strings = drive_strings[..len as usize].to_vec();
        drive_strings.push(0); // Add final null terminator

        let mut current_pos = 0;
        while current_pos < drive_strings.len() {
            // Find end of current string
            let mut end_pos = current_pos;
            while end_pos < drive_strings.len() && drive_strings[end_pos] != 0 {
                end_pos += 1;
            }

            // Process valid drive strings
            if end_pos > current_pos {
                let drive_path = &drive_strings[current_pos..end_pos];
                if !drive_path.is_empty() {
                    // Create null-terminated copy for Windows APIs
                    let mut path_with_null = drive_path.to_vec();
                    path_with_null.push(0);

                    // Variables for disk space information
                    let mut free_bytes_available: u64 = 0;
                    let mut total_bytes: u64 = 0;
                    let mut total_free_bytes: u64 = 0;

                    // Get disk space information
                    let result = unsafe {
                        GetDiskFreeSpaceExW(
                            CorePWSTR::from_raw(path_with_null.as_mut_ptr()),
                            Some(&mut free_bytes_available),
                            Some(&mut total_bytes),
                            Some(&mut total_free_bytes),
                        )
                    };

                    if result.is_ok() {
                        total += total_bytes;
                        free += free_bytes_available;
                    }
                }
            }

            // Move to next string
            current_pos = end_pos + 1;
        }
    }

    // Calculate used space (handle potential overflow)
    let used = if total >= free { total - free } else { 0 };

    DiskInfo {
        used: used as usize,
        free: free as usize,
        total: total as usize,
    }
}

// Loads disk information into the SystemInfo structure
pub fn load_disk_info(info: &mut SystemInfo) {
    let disk: DiskInfo = get_disk();
    insert_size("disk", disk.free, disk.used, disk.total, info);
}
