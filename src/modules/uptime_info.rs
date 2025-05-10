use super::SystemInfo;

#[cfg(target_os = "linux")]
fn get_uptime() -> u64 {
    use std::fs;

    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime_secs_str) = uptime_str.split_whitespace().next() {
            if let Ok(uptime_secs) = uptime_secs_str.parse::<f64>() {
                let uptime_secs = uptime_secs as u64;
                return uptime_secs;
            }
        }
    }

    0
}

#[cfg(target_os = "windows")]
fn get_uptime() -> u64 {
    use windows::Win32::System::SystemInformation::GetTickCount64;

    unsafe {
        // GetTickCount64 returns milliseconds since boot
        let millis = GetTickCount64();
        millis / 1000
    }
}

pub fn load_uptime_info(info: &mut SystemInfo) {
    let uptime = get_uptime();

    let days = uptime / 60 / 60 / 24;
    let hours = (uptime - days * 60 * 60 * 24) / 60 / 60;
    let minutes = (uptime - days * 60 * 60 * 24 - hours * 60 * 60) / 60;
    let seconds = uptime - days * 60 * 60 * 24 - hours * 60 * 60 - minutes * 60;

    let total_hours = days * 24 + hours;
    let total_minutes = total_hours * 60 + minutes;

    info.insert("uptime_days".to_string(), days.to_string());

    info.insert("uptime_hours".to_string(), hours.to_string());
    info.insert("uptime_hours_total".to_string(), total_hours.to_string());

    info.insert("uptime_mins".to_string(), minutes.to_string());
    info.insert("uptime_mins_total".to_string(), total_minutes.to_string());

    info.insert("uptime_secs".to_string(), seconds.to_string());
    info.insert("uptime_secs_total".to_string(), uptime.to_string());

    let mut uptime_str = String::new();

    if days > 0 {
        uptime_str.push_str(&format!("{}d ", days));
    }

    if hours > 0 {
        uptime_str.push_str(&format!("{}h ", hours));
    }

    if minutes > 0 {
        uptime_str.push_str(&format!("{}m ", minutes));
    }

    if seconds > 0 {
        uptime_str.push_str(&format!("{}s", seconds));
    }

    info.insert("uptime".to_string(), uptime_str);
}
