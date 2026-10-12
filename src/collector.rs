use crate::gpu::GpuReader;
use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
#[cfg(target_os = "linux")]
use std::process::Command;
use sysinfo::{Disks, System};

#[derive(Debug, Clone)]
pub struct CpuSnapshot {
    pub brand: String,
    pub usage: f32,
    pub freq_mhz: u64,
    pub cores: usize,
    pub per_core: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct MemSnapshot {
    pub total: u64,
    pub used: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone)]
pub struct GpuSnapshot {
    pub name: String,
    pub usage: Option<u32>,
    pub mem_used: Option<u64>,
    pub mem_total: Option<u64>,
    pub temp: Option<u32>,
    pub freq_mhz: Option<u64>,
    pub freq_max_mhz: Option<u64>,
    pub live: bool,
}

#[derive(Debug, Clone)]
pub struct DiskSnapshot {
    pub device: String,
    pub mount: String,
    pub fs: String,
    pub total: u64,
    pub available: u64,
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub hostname: String,
    pub username: String,
    pub os: String,
    pub kernel: String,
    pub uptime_secs: u64,
    pub load: (f64, f64, f64),
    pub cpu: CpuSnapshot,
    pub mem: MemSnapshot,
    pub gpus: Vec<GpuSnapshot>,
    pub disks: Vec<DiskSnapshot>,
}

pub struct Collector {
    sys: System,
    disks: Disks,
    nvml: Option<Nvml>,
    gpu_reader: GpuReader,
    gpu_fallback: Vec<String>,
}

impl Collector {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_usage();

        let disks = Disks::new_with_refreshed_list();
        let nvml = Nvml::init().ok();
        let gpu_fallback = detect_gpus_lspci();
        let gpu_reader = GpuReader::new(&gpu_fallback);

        Self {
            sys,
            disks,
            nvml,
            gpu_reader,
            gpu_fallback,
        }
    }

    pub fn refresh(&mut self) -> Snapshot {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
        self.disks.refresh(true);

        let cpu = self.cpu_snapshot();
        let mem = self.mem_snapshot();
        let load = System::load_average();
        let gpus = self.gpu_snapshots();

        let disks = self
            .disks
            .list()
            .iter()
            .filter(|d| d.total_space() > 0)
            .filter(|d| !is_pseudo_fs(&d.file_system().to_string_lossy()))
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                DiskSnapshot {
                    device: d.name().to_string_lossy().to_string(),
                    mount: d.mount_point().to_string_lossy().to_string(),
                    fs: d.file_system().to_string_lossy().to_string(),
                    total,
                    available,
                }
            })
            .collect();

        Snapshot {
            hostname: System::host_name().unwrap_or_else(|| "unknown".into()),
            username: current_user(),
            os: System::long_os_version()
                .or_else(System::name)
                .unwrap_or_else(|| "unknown".into()),
            kernel: System::kernel_version().unwrap_or_else(|| "unknown".into()),
            uptime_secs: System::uptime(),
            load: (load.one, load.five, load.fifteen),
            cpu,
            mem,
            gpus,
            disks,
        }
    }

    fn cpu_snapshot(&self) -> CpuSnapshot {
        let cpus = self.sys.cpus();
        let first = cpus.first();
        let brand = first
            .map(|c| c.brand().trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "CPU".into());
        let freq_mhz = first.map(|c| c.frequency()).unwrap_or(0);
        let per_core = cpus.iter().map(|c| c.cpu_usage()).collect();
        CpuSnapshot {
            brand,
            usage: self.sys.global_cpu_usage(),
            freq_mhz,
            cores: cpus.len(),
            per_core,
        }
    }

    fn mem_snapshot(&self) -> MemSnapshot {
        MemSnapshot {
            total: self.sys.total_memory(),
            used: self.sys.used_memory(),
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),
        }
    }

    fn gpu_snapshots(&mut self) -> Vec<GpuSnapshot> {
        if let Some(nvml) = &self.nvml {
            let count = nvml.device_count().unwrap_or(0);
            if count > 0 {
                return (0..count)
                    .filter_map(|i| nvml.device_by_index(i).ok())
                    .map(|dev| {
                        let name = dev.name().unwrap_or_else(|_| "NVIDIA GPU".into());
                        let usage = dev.utilization_rates().ok().map(|u| u.gpu);
                        let (mem_used, mem_total) = match dev.memory_info() {
                            Ok(m) => (Some(m.used), Some(m.total)),
                            Err(_) => (None, None),
                        };
                        let temp = dev.temperature(TemperatureSensor::Gpu).ok();
                        GpuSnapshot {
                            name,
                            usage,
                            mem_used,
                            mem_total,
                            temp,
                            freq_mhz: None,
                            freq_max_mhz: None,
                            live: true,
                        }
                    })
                    .collect();
            }
        }
        if self.gpu_reader.available() {
            return self.gpu_reader.read();
        }
        self.gpu_fallback
            .iter()
            .map(|name| GpuSnapshot {
                name: name.clone(),
                usage: None,
                mem_used: None,
                mem_total: None,
                temp: None,
                freq_mhz: None,
                freq_max_mhz: None,
                live: false,
            })
            .collect()
    }
}

fn is_pseudo_fs(fs: &str) -> bool {
    matches!(
        fs,
        "tmpfs"
            | "devtmpfs"
            | "devfs"
            | "proc"
            | "sysfs"
            | "cgroup"
            | "cgroup2"
            | "overlay"
            | "squashfs"
            | "autofs"
            | "devpts"
            | "securityfs"
            | "debugfs"
            | "tracefs"
            | "pstore"
            | "bpf"
            | "configfs"
            | "fusectl"
            | "hugetlbfs"
            | "mqueue"
            | "ramfs"
            | "efivarfs"
            | "nsfs"
            | "rpc_pipefs"
            | "selinuxfs"
            | "binfmt_misc"
            | "fuse.gvfsd-fuse"
            | "fuse.portal"
    )
}

fn current_user() -> String {
    if let Ok(user) = std::env::var("USER")
        && !user.is_empty()
    {
        return user;
    }
    if let Ok(user) = std::env::var("LOGNAME")
        && !user.is_empty()
    {
        return user;
    }
    "user".into()
}

#[cfg(target_os = "linux")]
fn detect_gpus_lspci() -> Vec<String> {
    let output = match Command::new("lspci").output() {
        Ok(o) => o,
        Err(_) => return Vec::new(),
    };
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines()
        .filter(|l| {
            l.contains("VGA compatible controller")
                || l.contains("3D controller")
                || l.contains("Display controller")
        })
        .filter_map(|l| l.split_once(": ").map(|(_, rest)| rest.trim().to_string()))
        .collect()
}

#[cfg(not(target_os = "linux"))]
fn detect_gpus_lspci() -> Vec<String> {
    Vec::new()
}
