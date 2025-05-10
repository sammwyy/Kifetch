use crate::utils::get_size;

use super::SystemInfo;

struct GpuInfo {
    pub model: String,
    pub vram: usize,
}

impl Default for GpuInfo {
    fn default() -> Self {
        GpuInfo {
            model: "Unknown".to_string(),
            vram: 0,
        }
    }
}

#[cfg(target_os = "linux")]
fn get_cpu() -> GpuInfo {
    use std::process::Command;

    let mut info = GpuInfo::default();

    if let Ok(output) = Command::new("lspci").args(["-v"]).output() {
        let output_str = String::from_utf8_lossy(&output.stdout);

        // Find GPU info
        for line in output_str.lines() {
            if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                if let Some(gpu_info) = line.split(':').nth(2) {
                    info.model = gpu_info.trim().to_string();
                    break;
                }
            }
        }
    }

    return info;
}

#[cfg(target_os = "windows")]
fn get_cpu() -> GpuInfo {
    use windows::Win32::Graphics::Dxgi::*;

    let mut info = GpuInfo::default();

    unsafe {
        if let Ok(factory) = CreateDXGIFactory::<IDXGIFactory>() {
            let mut i = 0;
            while let Ok(adapter) = factory.EnumAdapters(i) {
                if let Ok(desc) = adapter.GetDesc() {
                    let name = String::from_utf16_lossy(
                        &desc
                            .Description
                            .iter()
                            .take_while(|&&c| c != 0)
                            .cloned()
                            .collect::<Vec<u16>>(),
                    );

                    info.model = name;
                    info.vram = desc.DedicatedVideoMemory;
                    break;
                }

                i += 1;
            }
        }
    }

    info
}

pub fn load_gpu_info(info: &mut SystemInfo) {
    let gpu = get_cpu();
    info.insert("gpu_model".to_string(), gpu.model);

    let size = get_size(gpu.vram as f64);
    info.insert("gpu_vram".to_string(), format!("{:.2}", size.metric_value));
    info.insert("gpu_vram_metric".to_string(), size.metric);
}
