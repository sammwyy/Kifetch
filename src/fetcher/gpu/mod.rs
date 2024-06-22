use serde_derive::{Deserialize, Serialize};

mod nvidia;

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuFan {
    pub speed: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpuMemory {
    pub free: u64,
    pub total: u64,
    pub used: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gpu {
    pub name: String,
    pub arch: String,
    pub brand: String,
    pub graphic_clock: u32,
    pub memory_clock: u32,
    pub video_clock: u32,
    pub memory: GpuMemory,
    pub fans: Vec<GpuFan>,
    pub power_limit: u32,
    pub encoder_utilization: u32,
    pub bus_type: String,
    pub cores: u32,
    pub power_usage: u32,
    pub temperature: u32,
}

pub fn get_gpu() -> Option<Gpu> {
    let gpu = nvidia::get_nvidia();
    match gpu {
        Some(gpu) => Some(gpu),
        None => None,
    }
}

pub fn get_gpu_or_default() -> Gpu {
    match get_gpu() {
        Some(gpu) => gpu,
        None => Gpu {
            name: "Unknown".to_string(),
            arch: "Unknown".to_string(),
            brand: "Unknown".to_string(),
            graphic_clock: 0,
            memory_clock: 0,
            video_clock: 0,
            memory: GpuMemory {
                free: 0,
                total: 0,
                used: 0,
            },
            fans: vec![GpuFan { speed: 0 }],
            power_limit: 0,
            encoder_utilization: 0,
            bus_type: "Unknown".to_string(),
            cores: 0,
            power_usage: 0,
            temperature: 0,
        },
    }
}
