use super::SystemInfo;

struct NetInfo {
    pub ip: String,
    pub mac: String,
    pub iface: String,
    pub wifi_ssid: String,
    pub wifi_signal: String,
}

impl Default for NetInfo {
    fn default() -> Self {
        NetInfo {
            ip: "unknown".to_string(),
            mac: "unknown".to_string(),
            iface: "unknown".to_string(),
            wifi_ssid: "unknown".to_string(),
            wifi_signal: "unknown".to_string(),
        }
    }
}

#[cfg(target_os = "linux")]
fn get_net() -> NetInfo {
    use pnet::datalink;
    use std::process::Command;
    use std::str;

    // println!("[DEBUG] Starting network information collection on Linux");

    // Start with default values
    let mut net_info = NetInfo::default();

    // Get active network interfaces
    let interfaces = match datalink::interfaces() {
        interfaces if !interfaces.is_empty() => {
            // println!("[DEBUG] Found {} network interfaces", interfaces.len());
            interfaces
        }
        _ => {
            // println!("[DEBUG] No network interfaces found");
            return net_info;
        }
    };

    // Find the first active interface with an IP
    for iface in interfaces {
        // println!("[DEBUG] Checking interface: {}", iface.name);

        if iface.is_up() && !iface.ips.is_empty() && !iface.mac.unwrap().is_zero() {
            // println!("[DEBUG] Found active interface: {}", iface.name);

            // Found the primary interface
            for ip in iface.ips {
                if ip.is_ipv4() {
                    // println!("[DEBUG] Found IPv4 address: {}", ip.ip());
                    net_info.ip = ip.ip().to_string();
                    net_info.iface = iface.name.clone();
                    net_info.mac = iface.mac.unwrap_or_default().to_string();

                    // If it's a WiFi interface, try to get SSID and signal
                    if iface.name.starts_with("wl") {
                        // println!("[DEBUG] WiFi interface detected, attempting to get SSID");

                        // Try to get SSID
                        if let Ok(output) =
                            Command::new("iwgetid").arg("-r").arg(&iface.name).output()
                        {
                            if output.status.success() {
                                if let Ok(ssid) = str::from_utf8(&output.stdout) {
                                    // println!("[DEBUG] Found SSID: {}", ssid.trim());
                                    net_info.wifi_ssid = ssid.trim().to_string();
                                }
                            } else {
                                // println!("[DEBUG] Failed to get SSID with iwgetid");
                            }
                        }

                        // Try to get signal strength
                        // println!("[DEBUG] Attempting to get signal strength");
                        if let Ok(output) = Command::new("iwconfig").arg(&iface.name).output() {
                            if output.status.success() {
                                if let Ok(iwconfig_output) = str::from_utf8(&output.stdout) {
                                    if let Some(signal_idx) = iwconfig_output.find("Signal level=")
                                    {
                                        let signal_str = &iwconfig_output[signal_idx + 13..];
                                        if let Some(end_idx) = signal_str.find(' ') {
                                            net_info.wifi_signal =
                                                signal_str[..end_idx].to_string();
                                            /* println!(
                                                "[DEBUG] Found signal strength: {}",
                                                net_info.wifi_signal
                                            );
                                            */
                                        }
                                    }
                                }
                            } else {
                                // println!("[DEBUG] Failed to get signal strength with iwconfig");
                            }
                        }
                    }

                    // If we found an interface with IP, use it and finish
                    return net_info;
                }
            }
        }
    }

    // println!("[DEBUG] No suitable network interface found");
    net_info
}

#[cfg(target_os = "windows")]
fn get_net() -> NetInfo {
    use socket2::SockAddr;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use windows::Win32::Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS};
    use windows::Win32::NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GetBestInterfaceEx, GAA_FLAG_INCLUDE_GATEWAYS, IF_TYPE_IEEE80211,
        IP_ADAPTER_ADDRESSES_LH,
    };
    use windows::Win32::NetworkManagement::WiFi::{
        wlan_interface_state_connected, wlan_intf_opcode_current_connection, WlanCloseHandle,
        WlanEnumInterfaces, WlanOpenHandle, WlanQueryInterface, WLAN_API_VERSION_2_0,
        WLAN_CONNECTION_ATTRIBUTES, WLAN_INTERFACE_INFO_LIST, WLAN_OPCODE_VALUE_TYPE,
    };
    use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_IN};

    // println!("[DEBUG] Starting network information collection on Windows");

    let mut net_info = NetInfo::default();

    unsafe {
        // Create a socket to determine network route
        let google_dns = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
        let sock_addr = SockAddr::from(google_dns);
        let mut if_index: u32 = 0;

        // println!("[DEBUG] Finding best interface for Google DNS");
        let result = GetBestInterfaceEx(sock_addr.as_ptr() as *const _, &mut if_index);

        if result == ERROR_SUCCESS.0 {
            // println!("[DEBUG] Found best interface index: {}", if_index);

            // Get adapter information
            let mut buf_len: u32 = 15000;
            let mut buffer: Vec<u8> = Vec::with_capacity(buf_len as usize);

            loop {
                /*
                println!(
                    "[DEBUG] Attempting to get adapter addresses with buffer size: {}",
                    buf_len
                );
                */
                let result = GetAdaptersAddresses(
                    AF_INET.0.into(),
                    GAA_FLAG_INCLUDE_GATEWAYS,
                    None,
                    Some(buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
                    &mut buf_len,
                );

                if result == ERROR_SUCCESS.0 {
                    // println!("[DEBUG] Successfully retrieved adapter addresses");
                    buffer.set_len(buf_len as usize);
                    break;
                } else {
                    // println!("[DEBUG] Need larger buffer for adapter addresses");
                    buffer = Vec::with_capacity(buf_len as usize);
                    if result != ERROR_BUFFER_OVERFLOW.0 {
                        // println!("[DEBUG] Error getting adapter addresses: {}", result);
                        return net_info;
                    }
                }
            }

            // Iterate through adapters
            let mut current_addr = buffer.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
            while !current_addr.is_null() {
                let adapter = &*current_addr;

                // Check if this is the adapter we wanted
                let adapter_index = adapter.ZoneIndices[0];
                if adapter_index == if_index {
                    // println!("[DEBUG] Found matching adapter index");

                    // Interface name
                    if !adapter.FriendlyName.is_null() {
                        if let Ok(name) = adapter.FriendlyName.to_string() {
                            // println!("[DEBUG] Found interface name: {}", name);
                            net_info.iface = name;
                        }
                    }

                    // MAC address
                    if adapter.PhysicalAddressLength > 0 {
                        let mac = (0..adapter.PhysicalAddressLength)
                            .map(|i| format!("{:02x}", adapter.PhysicalAddress[i as usize]))
                            .collect::<Vec<String>>()
                            .join(":");
                        // println!("[DEBUG] Found MAC address: {}", mac);
                        net_info.mac = mac;
                    }

                    // IP address
                    let mut unicast_addr = adapter.FirstUnicastAddress;
                    while !unicast_addr.is_null() {
                        let addr = &*unicast_addr;
                        let socket_addr = addr.Address.lpSockaddr;

                        if !socket_addr.is_null() {
                            if (*socket_addr).sa_family == AF_INET {
                                let ipv4_addr = &*(socket_addr as *const SOCKADDR_IN);
                                let ip =
                                    Ipv4Addr::from(ipv4_addr.sin_addr.S_un.S_addr.to_ne_bytes());
                                // println!("[DEBUG] Found IP address: {}", ip);
                                net_info.ip = ip.to_string();
                                break;
                            }
                        }

                        unicast_addr = addr.Next;
                    }

                    // If it's WiFi, try to get SSID and signal strength
                    if adapter.IfType == IF_TYPE_IEEE80211 {
                        // println!("[DEBUG] WiFi adapter detected, attempting to get WiFi info");

                        // Try to get WiFi information
                        let mut handle = Default::default();
                        let client_version = WLAN_API_VERSION_2_0;
                        let mut negotiated_version = 0;

                        let result = WlanOpenHandle(
                            client_version,
                            None,
                            &mut negotiated_version,
                            &mut handle,
                        );

                        if result == ERROR_SUCCESS.0 {
                            // println!("[DEBUG] Successfully opened WLAN handle");

                            let mut iface_list_ptr = std::ptr::null_mut();
                            let result = WlanEnumInterfaces(handle, None, &mut iface_list_ptr);

                            if result == ERROR_SUCCESS.0 && !iface_list_ptr.is_null() {
                                // println!("[DEBUG] Successfully enumerated WLAN interfaces");

                                let iface_list =
                                    &*(iface_list_ptr as *const WLAN_INTERFACE_INFO_LIST);

                                for i in 0..iface_list.dwNumberOfItems {
                                    let iface_info = &iface_list.InterfaceInfo[i as usize];

                                    // If we find SSID and signal information, save it
                                    if iface_info.isState == wlan_interface_state_connected {
                                        // println!("[DEBUG] Found connected WiFi interface");

                                        let mut data_size = 0u32;
                                        let mut data_ptr = std::ptr::null_mut();
                                        let mut opcode_value_type = WLAN_OPCODE_VALUE_TYPE(0);

                                        let result = WlanQueryInterface(
                                            handle,
                                            &iface_info.InterfaceGuid,
                                            wlan_intf_opcode_current_connection,
                                            None,
                                            &mut data_size,
                                            &mut data_ptr,
                                            Some(&mut opcode_value_type),
                                        );

                                        if result == ERROR_SUCCESS.0 && !data_ptr.is_null() {
                                            let conn_attr =
                                                *(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES);
                                            let ssid_bytes =
                                                &conn_attr.wlanAssociationAttributes.dot11Ssid;
                                            let ssid = String::from_utf8_lossy(
                                                &ssid_bytes.ucSSID
                                                    [..ssid_bytes.uSSIDLength as usize],
                                            );
                                            net_info.wifi_ssid = ssid.to_string();
                                            net_info.wifi_signal = format!(
                                                "{} dBm",
                                                conn_attr
                                                    .wlanAssociationAttributes
                                                    .wlanSignalQuality
                                            );
                                            // wlanSignalQuality está en 0-100, puedes convertirlo a dBm si querés
                                        }
                                    }
                                }
                            } else {
                                // println!("[DEBUG] Failed to enumerate WLAN interfaces");
                            }
                        } else {
                            // println!("[DEBUG] Failed to open WLAN handle");
                        }

                        WlanCloseHandle(handle, None);
                    }

                    break;
                }

                current_addr = adapter.Next;
            }
        } else {
            // println!("[DEBUG] Failed to find best interface, error: {}", result);
        }
    }

    net_info
}

pub fn load_net_info(info: &mut SystemInfo) {
    // println!("[DEBUG] Loading network information into SystemInfo");
    let net = get_net();

    info.insert("net_ip".to_string(), net.ip);
    info.insert("net_mac".to_string(), net.mac);
    info.insert("net_iface".to_string(), net.iface);
    info.insert("net_wifi_ssid".to_string(), net.wifi_ssid);
    info.insert("net_wifi_signal".to_string(), net.wifi_signal);

    // println!("[DEBUG] Network information loaded successfully");
}
