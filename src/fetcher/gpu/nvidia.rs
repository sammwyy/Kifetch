use nvml_wrapper::{
    enum_wrappers::device::{Brand, Clock, ClockId, TemperatureSensor},
    enums::device::BusType,
    Nvml,
};

use super::{Gpu, GpuFan, GpuMemory};

pub fn get_nvidia() -> Option<Gpu> {
    // Initialize NVML
    let nvml = Nvml::init();
    if nvml.is_err() {
        return None;
    }
    let nvml = nvml.unwrap();

    // Get the first device
    let device = nvml.device_by_index(0);
    if device.is_err() {
        return None;
    }
    let device = device.unwrap();

    // Get name
    let name = device.name().unwrap_or("Unknown".to_string());

    // Get arch
    let arch = if device.architecture().is_ok() {
        device.architecture().unwrap().to_string()
    } else {
        "Unknown".to_string()
    };

    // Get brand
    let brand = if device.brand().is_ok() {
        brand_to_str(device.brand().unwrap())
    } else {
        "Unknown".to_string()
    };

    // Get clock speeds
    let graphic_clock = device.clock(Clock::Graphics, ClockId::Current).unwrap_or(0);
    let memory_clock = device.clock(Clock::Memory, ClockId::Current).unwrap_or(0);
    let video_clock = device.clock(Clock::Video, ClockId::Current).unwrap_or(0);

    // Get memory
    let memory = if device.memory_info().is_ok() {
        let memory = device.memory_info().unwrap();
        GpuMemory {
            free: memory.free,
            total: memory.total,
            used: memory.used,
        }
    } else {
        GpuMemory {
            free: 0,
            total: 0,
            used: 0,
        }
    };

    // Get fan speed.
    let mut fans = Vec::new();
    for i in 0..device.num_fans().unwrap_or(0) {
        let speed = device.fan_speed(i).unwrap_or(0);
        let fan = GpuFan { speed };
        fans.push(fan);
    }

    // Get power limit
    let power_limit = device.enforced_power_limit().unwrap_or(0);

    // Get encoder utilization.
    let encoder_utilization = if device.encoder_utilization().is_ok() {
        device.encoder_utilization().unwrap().utilization
    } else {
        0
    };

    // Get bus type.
    let bus_type = if device.pci_info().is_ok() {
        bus_to_str(device.bus_type().unwrap())
    } else {
        "Unknown".to_string()
    };

    let cores = device.num_cores().unwrap_or(0);
    let power_usage = device.power_usage().unwrap_or(0);
    let temperature = device.temperature(TemperatureSensor::Gpu).unwrap_or(0);

    Some(Gpu {
        name,
        arch,
        brand,
        graphic_clock,
        memory_clock,
        video_clock,
        memory,
        fans,
        power_limit,
        encoder_utilization,
        bus_type,
        cores,
        power_usage,
        temperature,
    })
}

fn bus_to_str(bus: BusType) -> String {
    match bus {
        BusType::Agp => "AGP".to_string(),
        BusType::Pci => "PCI".to_string(),
        BusType::Fpci => "FPCI".to_string(),
        BusType::Pcie => "PCIe".to_string(),
        BusType::Unknown => "Unknown".to_string(),
    }
}

fn brand_to_str(brand: Brand) -> String {
    match brand {
        Brand::GeForce => "GeForce".to_string(),
        Brand::Quadro => "Quadro".to_string(),
        Brand::Tesla => "Tesla".to_string(),
        Brand::NVS => "NVS".to_string(),
        Brand::GRID => "GRID".to_string(),
        Brand::CloudGaming => "Cloud Gaming".to_string(),
        Brand::GeForceRTX => "GeForce RTX".to_string(),
        Brand::TitanRTX => "Titan RTX".to_string(),
        Brand::QuadroRTX => "Quadro RTX".to_string(),
        Brand::Unknown => "Unknown".to_string(),
        Brand::NvidiaRTX => "Nvidia RTX".to_string(),
        Brand::VPC => "VPC".to_string(),
        Brand::VCS => "VCS".to_string(),
        Brand::VWS => "VWS".to_string(),
        Brand::VApps => "VApps".to_string(),
        Brand::VGaming => "VGaming".to_string(),
        Brand::Nvidia => "Nvidia".to_string(),
        Brand::Titan => "Titan".to_string(),
    }
}
