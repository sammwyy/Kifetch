use gpu::Gpu;
use serde_derive::{Deserialize, Serialize};
use sysinfo::{Disks, Networks, System};

mod gpu;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemCPU {
    pub cores: u64,
    pub threads: u64,
    pub brand: String,
    pub freq: u64,
    pub usage: f32,
    pub vendor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDisk {
    pub available_space: u64,
    pub file_system: String,
    pub is_removable: bool,
    pub kind: String,
    pub mount_point: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMemory {
    pub total: u64,
    pub used: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    pub version: String,
    pub kernel: String,
    pub host: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemNetworkInterfaceStats {
    pub data: u64,
    pub total: u64,
    pub errors: u64,
    pub packets: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemNetworkInterface {
    pub name: String,
    pub mac: String,
    pub received: SystemNetworkInterfaceStats,
    pub transmitted: SystemNetworkInterfaceStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemUptime {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemFetchResult {
    pub boot_time: u64,
    pub cpu: SystemCPU,
    pub disks: Vec<SystemDisk>,
    pub gpu: Gpu,
    pub memory: SystemMemory,
    pub networking: Vec<SystemNetworkInterface>,
    pub os: SystemInfo,
    pub processes: u64,
    pub swap: SystemMemory,
    pub uptime: SystemUptime,
}

pub struct Fetcher {
    system: System,
}

impl Fetcher {
    pub fn new() -> Self {
        let system = System::new();

        Fetcher { system }
    }

    pub fn boot_time(&self) -> u64 {
        System::boot_time()
    }

    pub fn cpu(&mut self) -> SystemCPU {
        self.system.refresh_cpu();

        let cores = self.system.physical_core_count().unwrap_or(0) as u64;
        let threads = self.system.cpus().len() as u64;

        let mut freq = 0;
        let mut brand = "Unknown".to_string();
        let mut usage = 0.0;
        let mut vendor = "Unknown".to_string();

        let first_core = self.system.cpus().get(0);

        if first_core.is_some() {
            let first_core = first_core.unwrap();
            freq = first_core.frequency() as u64;
            brand = first_core.brand().to_string();
            usage = first_core.cpu_usage();
            vendor = first_core.vendor_id().to_string();
        }

        SystemCPU {
            cores,
            freq,
            brand,
            usage,
            threads,
            vendor,
        }
    }

    pub fn disks(&self) -> Vec<SystemDisk> {
        let raw_disks = Disks::new_with_refreshed_list();
        let mut disks = Vec::new();

        for raw_disk in raw_disks.iter() {
            let available_space = raw_disk.available_space();
            let file_system = raw_disk.file_system().to_str().unwrap().to_string();
            let is_removable = raw_disk.is_removable();
            let kind = raw_disk.kind().to_string();
            let mount_point = raw_disk.mount_point().to_str().unwrap().to_string();
            let name = raw_disk.name().to_str().unwrap().to_string();

            let disk = SystemDisk {
                available_space,
                file_system,
                is_removable,
                kind,
                mount_point,
                name,
            };

            disks.push(disk);
        }

        disks
    }

    pub fn gpu(&self) -> Gpu {
        gpu::get_gpu_or_default()
    }

    pub fn network(&self) -> Vec<SystemNetworkInterface> {
        let mut interfaces = Vec::new();

        let networks = Networks::new_with_refreshed_list();
        for network in networks.iter() {
            let name = network.0.to_string();
            let mac = network.1.mac_address().to_string();
            let received = SystemNetworkInterfaceStats {
                data: network.1.received(),
                total: network.1.total_received(),
                errors: network.1.total_errors_on_received(),
                packets: network.1.total_packets_received(),
            };
            let transmitted = SystemNetworkInterfaceStats {
                data: network.1.transmitted(),
                total: network.1.total_transmitted(),
                errors: network.1.total_errors_on_transmitted(),
                packets: network.1.total_packets_transmitted(),
            };

            interfaces.push(SystemNetworkInterface {
                name,
                mac,
                received,
                transmitted,
            });
        }

        interfaces
    }

    pub fn memory(&mut self) -> SystemMemory {
        self.system.refresh_memory();

        let total = self.system.total_memory();
        let used = self.system.used_memory();

        SystemMemory { total, used }
    }

    pub fn os(&self) -> SystemInfo {
        SystemInfo {
            name: System::name().unwrap_or("Unknown".to_string()),
            version: System::os_version().unwrap_or("Unknown".to_string()),
            kernel: System::kernel_version().unwrap_or("Unknown".to_string()),
            host: System::host_name().unwrap_or("Unknown".to_string()),
        }
    }

    pub fn processes(&self) -> u64 {
        self.system.processes().len() as u64
    }

    pub fn swap(&mut self) -> SystemMemory {
        self.system.refresh_memory();

        let total = self.system.total_swap();
        let used = self.system.used_swap();

        SystemMemory { total, used }
    }

    pub fn uptime(&self) -> SystemUptime {
        let total = System::uptime();
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        let seconds = total % 60;

        SystemUptime {
            hours,
            minutes,
            seconds,
            total,
        }
    }

    pub fn fetch(&mut self) -> SystemFetchResult {
        SystemFetchResult {
            boot_time: self.boot_time(),
            cpu: self.cpu(),
            disks: self.disks(),
            gpu: self.gpu(),
            memory: self.memory(),
            networking: self.network(),
            os: self.os(),
            processes: self.processes(),
            swap: self.swap(),
            uptime: self.uptime(),
        }
    }

    pub fn fetch_json(&mut self) -> String {
        serde_json::to_string(&self.fetch()).unwrap()
    }
}
