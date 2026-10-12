use crate::collector::GpuSnapshot;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
enum Driver {
    Intel,
    Amd,
    Other,
}

struct Card {
    base: PathBuf,
    device: PathBuf,
    driver: Driver,
    name: String,
}

pub struct GpuReader {
    cards: Vec<Card>,
    last: HashMap<PathBuf, (u64, Instant)>,
}

impl GpuReader {
    pub fn new(fallback_names: &[String]) -> Self {
        let mut cards = Vec::new();
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            let mut idx = 0;
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if !is_card_dir(&name) {
                    continue;
                }
                let base = entry.path();
                let device = base.join("device");
                let vendor = read_hex(&device.join("vendor")).unwrap_or(0);
                let driver = match vendor {
                    0x8086 => Driver::Intel,
                    0x1002 | 0x1022 => Driver::Amd,
                    _ => Driver::Other,
                };
                let gpu_name = fallback_names
                    .get(idx)
                    .cloned()
                    .unwrap_or_else(|| default_name(driver, &name));
                idx += 1;
                cards.push(Card {
                    base,
                    device,
                    driver,
                    name: gpu_name,
                });
            }
        }
        Self {
            cards,
            last: HashMap::new(),
        }
    }

    pub fn available(&self) -> bool {
        !self.cards.is_empty()
    }

    pub fn read(&mut self) -> Vec<GpuSnapshot> {
        let mut out = Vec::with_capacity(self.cards.len());
        for card in &self.cards {
            let snap = match card.driver {
                Driver::Intel => read_intel(card, Instant::now(), &mut self.last),
                Driver::Amd => read_amd(card),
                Driver::Other => GpuSnapshot {
                    name: card.name.clone(),
                    usage: None,
                    mem_used: None,
                    mem_total: None,
                    temp: hwmon_temp(&card.device),
                    freq_mhz: None,
                    freq_max_mhz: None,
                    live: false,
                },
            };
            out.push(snap);
        }
        out
    }
}

fn read_intel(card: &Card, now: Instant, last: &mut HashMap<PathBuf, (u64, Instant)>) -> GpuSnapshot {
    let gt = find_gt_dir(&card.base);
    let freq_mhz = gt
        .as_ref()
        .and_then(|g| read_u64(&g.join("rps_cur_freq_mhz")));
    let freq_max_mhz = gt
        .as_ref()
        .and_then(|g| read_u64(&g.join("rps_max_freq_mhz")));

    let usage = gt.as_ref().and_then(|g| {
        let rc6 = read_u64(&g.join("rc6_residency_ms"))?;
        let prev = last.insert(g.clone(), (rc6, now));
        let (prev_rc6, prev_t) = prev?;
        let dt = now.duration_since(prev_t).as_millis() as f64;
        if dt <= 0.0 {
            return None;
        }
        let drc6 = rc6.saturating_sub(prev_rc6) as f64;
        Some((100.0 * (1.0 - (drc6 / dt).clamp(0.0, 1.0)).clamp(0.0, 1.0)) as u32)
    });

    GpuSnapshot {
        name: card.name.clone(),
        usage,
        mem_used: None,
        mem_total: None,
        temp: hwmon_temp(&card.device),
        freq_mhz,
        freq_max_mhz,
        live: usage.is_some() || freq_mhz.is_some(),
    }
}

fn read_amd(card: &Card) -> GpuSnapshot {
    let usage = read_u64(&card.device.join("gpu_busy_percent")).map(|v| v as u32);
    let mem_used = read_u64(&card.device.join("mem_info_vram_used"));
    let mem_total = read_u64(&card.device.join("mem_info_vram_total"));
    let freq_mhz = hwmon_freq(&card.device);
    GpuSnapshot {
        name: card.name.clone(),
        usage,
        mem_used,
        mem_total,
        temp: hwmon_temp(&card.device),
        freq_mhz,
        freq_max_mhz: None,
        live: usage.is_some() || mem_total.is_some(),
    }
}

fn is_card_dir(name: &str) -> bool {
    match name.strip_prefix("card") {
        Some(rest) => !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit()),
        None => false,
    }
}

fn default_name(driver: Driver, card: &str) -> String {
    match driver {
        Driver::Intel => format!("Intel Graphics ({card})"),
        Driver::Amd => format!("AMD Radeon ({card})"),
        Driver::Other => format!("GPU ({card})"),
    }
}

fn find_gt_dir(base: &Path) -> Option<PathBuf> {
    let gt = base.join("gt");
    if gt.is_dir() {
        if gt.join("rc6_residency_ms").exists() {
            return Some(gt);
        }
        if let Ok(entries) = std::fs::read_dir(&gt) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.join("rc6_residency_ms").exists() {
                    return Some(p);
                }
            }
        }
    }
    let tile = base.join("tile0/gt0");
    if tile.join("rc6_residency_ms").exists() {
        return Some(tile);
    }
    None
}

fn read_u64(path: &Path) -> Option<u64> {
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

fn read_hex(path: &Path) -> Option<u16> {
    let s = std::fs::read_to_string(path).ok()?;
    let s = s.trim();
    let s = s.strip_prefix("0x").unwrap_or(s);
    u16::from_str_radix(s, 16).ok()
}

fn hwmon_freq(device: &Path) -> Option<u64> {
    let hwmon = device.join("hwmon");
    let entries = std::fs::read_dir(hwmon).ok()?;
    for entry in entries.flatten() {
        let input = entry.path().join("freq1_input");
        if let Some(v) = read_u64(&input) {
            return Some(v / 1_000_000);
        }
    }
    None
}

fn hwmon_temp(device: &Path) -> Option<u32> {
    let hwmon = device.join("hwmon");
    let entries = std::fs::read_dir(hwmon).ok()?;
    for entry in entries.flatten() {
        let input = entry.path().join("temp1_input");
        if let Some(v) = read_u64(&input) {
            return Some((v / 1000) as u32);
        }
    }
    None
}
