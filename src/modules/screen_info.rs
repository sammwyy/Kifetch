use super::SystemInfo;

// ScreenInfo struct to hold display information
// Consider adding doc comments (///) to document each field
struct ScreenInfo {
    width: usize,        // Screen width in pixels
    height: usize,       // Screen height in pixels
    refresh_rate: usize, // Refresh rate in Hz
}

impl Default for ScreenInfo {
    fn default() -> Self {
        ScreenInfo {
            width: 0,
            height: 0,
            refresh_rate: 0,
        }
    }
}

#[cfg(target_os = "linux")]
fn get_screen() -> ScreenInfo {
    use std::process::Command;

    // Execute xrandr command to get display information
    let output = Command::new("xrandr")
        .arg("--current")
        .output()
        .expect("Failed to execute xrandr");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();

    let mut found_connected = false;

    while let Some(line) = lines.next() {
        // Look for connected display
        if line.contains(" connected ") {
            found_connected = true;
            continue;
        }

        // If we're in a connected display block, look for resolution info
        if found_connected && line.trim().starts_with(char::is_numeric) {
            // Line format example: "   1920x1080     59.96*+"
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[1].contains('*') {
                let res_parts: Vec<&str> = parts[0].split('x').collect();
                if res_parts.len() == 2 {
                    if let (Ok(w), Ok(h)) =
                        (res_parts[0].parse::<usize>(), res_parts[1].parse::<usize>())
                    {
                        // Clean up refresh rate string (remove * and +)
                        let rate_str = parts[1].trim_end_matches(|c| c == '*' || c == '+');
                        if let Ok(rate) = rate_str.parse::<f32>() {
                            return ScreenInfo {
                                width: w,
                                height: h,
                                refresh_rate: rate.round() as usize,
                            };
                        }
                    }
                }
            }
        }

        // If we encounter a disconnected display, exit the block
        if line.contains("disconnected") {
            found_connected = false;
        }
    }

    ScreenInfo::default()
}

#[cfg(target_os = "windows")]
fn get_screen() -> ScreenInfo {
    use windows::Win32::Graphics::Gdi::{EnumDisplaySettingsW, DEVMODEW, ENUM_CURRENT_SETTINGS};

    let mut devmode = DEVMODEW::default();
    devmode.dmSize = std::mem::size_of::<DEVMODEW>() as u16;

    unsafe {
        if EnumDisplaySettingsW(None, ENUM_CURRENT_SETTINGS, &mut devmode).as_bool() {
            return ScreenInfo {
                width: devmode.dmPelsWidth as usize,
                height: devmode.dmPelsHeight as usize,
                refresh_rate: devmode.dmDisplayFrequency as usize,
            };
        }
    }

    ScreenInfo::default()
}

/// Loads screen information into the SystemInfo structure
/// Adds screen_width, screen_height, and screen_refresh_rate fields
pub fn load_screen_info(info: &mut SystemInfo) {
    let screen = get_screen();

    info.insert("screen_width".to_string(), screen.width.to_string());
    info.insert("screen_height".to_string(), screen.height.to_string());
    info.insert(
        "screen_refresh_rate".to_string(),
        screen.refresh_rate.to_string(),
    );
}
