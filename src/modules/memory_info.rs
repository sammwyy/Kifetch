use crate::utils::insert_size;

use super::SystemInfo;

struct MemoryInfo {
    total: usize,
    free: usize,
    available: Option<usize>,
}

#[cfg(target_os = "linux")]
fn get_memory() -> MemoryInfo {
    use std::fs;

    let mut total_kb: usize = 0;
    let mut free_kb: usize = 0;
    let mut available_kb: Option<usize> = None;

    if let Ok(mem_info) = fs::read_to_string("/proc/meminfo") {
        for line in mem_info.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if line.starts_with("MemFree:") {
                free_kb = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
            } else if line.starts_with("MemAvailable:") {
                available_kb = line.split_whitespace().nth(1).and_then(|s| s.parse().ok());
            }
        }
    };

    MemoryInfo {
        total: total_kb * 1024,
        free: free_kb * 1024,
        available: available_kb,
    }
}

#[cfg(target_os = "windows")]
fn get_memory() -> MemoryInfo {
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

    unsafe {
        let mut status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };

        if GlobalMemoryStatusEx(&mut status).is_ok() {
            MemoryInfo {
                total: (status.ullTotalPhys) as usize,
                free: (status.ullAvailPhys) as usize,
                available: Some((status.ullAvailPhys) as usize),
            }
        } else {
            MemoryInfo {
                total: 0,
                free: 0,
                available: None,
            }
        }
    }
}

pub fn load_memory_info(info: &mut SystemInfo) {
    let memory = get_memory();
    let used = memory.total - memory.free;
    insert_size("memory", memory.free, used, memory.total, info);

    if memory.available.is_some() {
        let available = memory.available.unwrap();
        let used = memory.total - available;
        insert_size("memory_available", available, used, memory.total, info);
    } else {
        info.insert("memory_available_free".to_string(), "N/A".to_string());
        info.insert("memory_available_used".to_string(), "N/A".to_string());
        info.insert("memory_available_total".to_string(), "N/A".to_string());
    }
}
