use std::env;
use std::env::consts::{ARCH, OS};

use super::SystemInfo;

pub fn load_env_info(info: &mut SystemInfo) {
    // Helper function to get environment variable or fallback to "unknown"
    let get_var = |key: &str| env::var(key).unwrap_or_else(|_| "unknown".to_string());

    // Get shell (SHELL on Unix, ComSpec on Windows)
    let shell_var = if cfg!(windows) {
        get_var("ComSpec") // e.g., C:\Windows\System32\cmd.exe
    } else {
        get_var("SHELL") // e.g., /bin/bash
    };

    // Extract shell name (basename only)
    let shell_name = shell_var
        .split(['/', '\\'])
        .last()
        .unwrap_or(&shell_var)
        .to_string();
    info.insert("env_shell".to_string(), shell_name);

    // Get current username
    let username = get_var(if cfg!(windows) { "USERNAME" } else { "USER" });
    info.insert("env_username".to_string(), username);

    // Get home directory
    let home = get_var("HOME"); // On Windows, USERPROFILE could be an alternative
    info.insert("env_home".to_string(), home);

    // Get system language
    let lang = get_var("LANG");
    info.insert("env_lang".to_string(), lang);

    // Get terminal (only on Unix-like systems)
    if !cfg!(windows) {
        let term = get_var("TERM");
        info.insert("env_term".to_string(), term);
    }

    // Get default editor
    let editor = get_var("EDITOR");
    info.insert("env_editor".to_string(), editor);

    // Get X11/Wayland display (only on Unix)
    if !cfg!(windows) {
        let display = get_var("DISPLAY");
        info.insert("env_display".to_string(), display);
    }

    // Get current desktop session (e.g., GNOME, KDE, etc.)
    let session = get_var("XDG_SESSION_DESKTOP");
    info.insert("env_session".to_string(), session);

    // Architecture and OS (from build constants)
    info.insert("env_arch".to_string(), ARCH.to_string());
    info.insert("env_os".to_string(), OS.to_string());
}
