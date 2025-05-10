# Kifetch

## Modules

- **CPU:**
  - `cpu_model` - CPU model name
  - `cpu_cores` - Number of CPU cores
  - `cpu_threads` - Number of CPU threads
  - `cpu_freq_mhz` - CPU frequency in MHz
  - `cpu_freq_ghz` - CPU frequency in GHz

- **GPU:**
  - `gpu_model` - GPU model name
  - `gpu_vram` - GPU VRAM (In bytes, kb, mb, gb or tb)
  - `gpu_vram_metric` - GPU VRAM metric (e.g. GB, TB, TB, etc.)

- **Kernel:**
  - `kernel_version` - Kernel version

- **Memory:**
  - `memory_total` - Total memory (In bytes, kb, mb, gb or tb)
  - `memory_free` - Free memory (In bytes, kb, mb, gb or tb)
  - `memory_used` - Used memory (In bytes, kb, mb, gb or tb)
  - `memory_percentage` - Memory usage percentage
  - `memory_metric` - Memory metric (e.g. GB, TB, TB, etc.)

- **OS:**
  - `os_name` - OS name
  - `os_version` - OS version
  - `os_hostname` - OS hostname

- **Packages:**
  - `package_manager` - The primary package manager installed on the system
  - `packages_total` - Total number of packages installed on the system
  - `packages` - Number of packages installed on the primary package manager
  - `packages_<manager>` - Number of packages installed on the specified package manager

- **Environment:**
  - `env_shell` - The primary shell used on the system
  - `env_username` - Get current username
  - `env_home` - Get home directory
  - `env_lang` - Get system language
  - `env_term` - Get terminal (only on Unix-like systems)
  - `env_editor` - Get default editor
  - `env_display` - Get X11/Wayland display (only on Unix)
  - `env_session` - Get current desktop session (e.g., GNOME, KDE, etc.)
  - `env_arch` - The architecture of the system
  - `env_os` - The operating system

- **Uptime:**
  - `uptime` - Uptime in string format (e.g. 1d 2h 3m 4s)
  - `uptime_days` - Uptime in days
  - `uptime_hours` - Uptime in hours
  - `uptime_hours_total` - Total uptime in hours
  - `uptime_mins` - Uptime in minutes
  - `uptime_mins_total` - Total uptime in minutes
  - `uptime_secs` - Uptime in seconds
  - `uptime_secs_total` - Total uptime in seconds

- **Screen:**
  - `screen_width` - The primary screen width
  - `screen_height` - The primary screen height
  - `screen_refresh_rate` - The primary screen refresh rate
  
- **Disk:**
  - `disk_total` - Total disk space (In bytes, kb, mb, gb or tb)
  - `disk_free` - Free disk space (In bytes, kb, mb, gb or tb)
  - `disk_used` - Used disk space (In bytes, kb, mb, gb or tb)
  - `disk_percentage` - Disk usage percentage
  - `disk_metric` - Disk metric (e.g. GB, TB, TB, etc.)

- **Network:**
  - `net_iface` - The primary network interface
  - `net_ip` - The primary network interface IP address
  - `net_mac` - The primary network interface MAC address
  - `net_wifi_ssid` - The primary network interface WiFi SSID
  - `net_wifi_signal` - The primary network interface WiFi signal strength
