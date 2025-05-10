use super::SystemInfo;

struct CpuInfo {
    pub model: String,
    pub cores: usize,
    pub threads: usize,
    pub freq_mhz: String,
    pub freq_ghz: String,
}

impl Default for CpuInfo {
    fn default() -> Self {
        CpuInfo {
            model: String::new(),
            cores: 0,
            threads: 0,
            freq_mhz: String::new(),
            freq_ghz: String::new(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_cpu() -> CpuInfo {
    use std::fs;

    let mut info = CpuInfo::default();

    if let Ok(cpu_info) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpu_info.lines() {
            if line.starts_with("model name") {
                if let Some(model) = line.split(':').nth(1) {
                    info.model = model.trim().to_string();
                    break;
                }
            }
        }

        // Get number of cores
        info.cores = cpu_info
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count();
    }

    // Get CPU frequency
    if let Ok(freq_info) =
        fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
    {
        if let Ok(freq) = freq_info.trim().parse::<u64>() {
            let freq_mhz = freq / 1000;
            info.freq_mhz = freq_mhz.to_string();
            info.freq_ghz = format!("{:.2}", freq_mhz as f64 / 1000.0);
        }
    }

    return info;
}

#[cfg(target_os = "windows")]
fn get_cpu() -> CpuInfo {
    use std::mem::zeroed;
    use windows::Win32::System::{
        SystemInformation::{
            GetLogicalProcessorInformation, GetSystemCpuSetInformation, GetSystemInfo,
            RelationProcessorCore, SYSTEM_CPU_SET_INFORMATION,
            SYSTEM_LOGICAL_PROCESSOR_INFORMATION,
        },
        Threading::GetCurrentProcess,
    };
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let mut info = CpuInfo::default();

    unsafe {
        // Get basic info
        let mut system_info = zeroed();
        GetSystemInfo(&mut system_info);

        // Get number of physical cores
        let mut buffer_size = 0u32;
        let process_handle = GetCurrentProcess();

        // Retrieve buffer size
        let cpu_set_info: Option<*mut SYSTEM_CPU_SET_INFORMATION> = Some(zeroed());

        let _ = GetSystemCpuSetInformation(
            cpu_set_info,
            0,
            &mut buffer_size,
            Some(process_handle),
            None,
        );

        if buffer_size == 0 {
            println!("Failed to get buffer size");
            return info;
        }

        let mut buffer = vec![0u8; buffer_size as usize];
        if !GetSystemCpuSetInformation(
            Some(buffer.as_mut_ptr() as *mut SYSTEM_CPU_SET_INFORMATION),
            buffer_size,
            &mut buffer_size,
            Some(process_handle),
            None,
        )
        .as_bool()
        {
            println!("Failed to get buffer");
            return info;
        }

        // Count physical cores
        let mut length = 0;
        let _ = GetLogicalProcessorInformation(Some(std::ptr::null_mut()), &mut length);

        if length > 0 {
            let count = length / size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>() as u32;
            let mut buffer = vec![zeroed::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>(); count as usize];

            if GetLogicalProcessorInformation(Some(buffer.as_mut_ptr()), &mut length).is_ok() {
                for slpi in buffer {
                    if slpi.Relationship == RelationProcessorCore {
                        // Each physical core has one processor
                        info.cores += 1;

                        // Count logical processors (threads) from the processor mask
                        let mask = slpi.ProcessorMask;
                        info.threads += mask.count_ones() as usize;
                    }
                }
            }
        }

        // Get frequency and CPU name from registry
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(cpu_key) = hklm.open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")
        {
            let cpu_value: Result<String, _> = cpu_key.get_value("ProcessorNameString");
            if let Ok(model) = cpu_value {
                info.model = model.trim().to_string();
            }

            // Read CPU frequency
            let freq_value: Result<u32, _> = cpu_key.get_value("~MHz");
            if let Ok(freq_mhz) = freq_value {
                info.freq_mhz = format!("{}", freq_mhz);
                info.freq_ghz = format!("{:.2}", freq_mhz as f64 / 1000.0);
            }
        }
    }

    return info;
}

pub fn load_cpu_info(info: &mut SystemInfo) {
    let cpu = get_cpu();
    info.insert("cpu_model".to_string(), cpu.model);
    info.insert("cpu_cores".to_string(), cpu.cores.to_string());
    info.insert("cpu_threads".to_string(), cpu.threads.to_string());
    info.insert("cpu_freq_mhz".to_string(), cpu.freq_mhz);
    info.insert("cpu_freq_ghz".to_string(), cpu.freq_ghz);
}
