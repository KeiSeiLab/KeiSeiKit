//! GPU detection via `system_profiler SPDisplaysDataType -json`.
//!
//! Apple Silicon machines report a single integrated GPU under
//! `SPDisplaysDataType[0]` with `sppci_model = "Apple M2 Pro"` (etc) and
//! `sppci_cores = "19"`. Intel iMacs / MacBook Pros surface either an
//! Intel-integrated entry or a discrete AMD/NVIDIA entry (sometimes both;
//! we pick the discrete one as the more capable).

use crate::profile::GpuInfo;
use crate::runner::Runner;
use serde_json::Value;

pub fn detect_gpu(runner: &dyn Runner) -> GpuInfo {
    let raw = match runner.run("system_profiler", &["SPDisplaysDataType", "-json"]) {
        Ok(s) => s,
        Err(_) => return GpuInfo::None,
    };
    let v: Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(_) => return GpuInfo::None,
    };
    parse_gpu_root(&v)
}

fn parse_gpu_root(v: &Value) -> GpuInfo {
    let arr = match v.get("SPDisplaysDataType").and_then(|x| x.as_array()) {
        Some(a) if !a.is_empty() => a,
        _ => return GpuInfo::None,
    };
    let mut best: Option<GpuInfo> = None;
    for item in arr {
        let candidate = classify_display_entry(item);
        best = Some(match best {
            None => candidate,
            Some(prev) => prefer(prev, candidate),
        });
    }
    best.unwrap_or(GpuInfo::None)
}

/// Apple integrated > Discrete > IntelIntegrated > None ordering.
fn prefer(a: GpuInfo, b: GpuInfo) -> GpuInfo {
    if score(&b) > score(&a) {
        b
    } else {
        a
    }
}

fn score(g: &GpuInfo) -> u8 {
    match g {
        GpuInfo::AppleIntegrated { .. } => 4,
        GpuInfo::Discrete { .. } => 3,
        GpuInfo::IntelIntegrated { .. } => 2,
        GpuInfo::None => 0,
    }
}

fn classify_display_entry(item: &Value) -> GpuInfo {
    let name = read_str(item, "sppci_model").to_string();
    let cores = read_str(item, "sppci_cores").parse::<u32>().unwrap_or(0);
    let bus = read_str(item, "sppci_bus");
    if name.is_empty() {
        return GpuInfo::None;
    }
    if name.starts_with("Apple") {
        return GpuInfo::AppleIntegrated { cores, name };
    }
    if name.to_ascii_lowercase().contains("intel") {
        return GpuInfo::IntelIntegrated { name };
    }
    if !bus.is_empty() {
        return GpuInfo::Discrete { name };
    }
    GpuInfo::Discrete { name }
}

fn read_str<'a>(item: &'a Value, key: &str) -> &'a str {
    item.get(key).and_then(|x| x.as_str()).unwrap_or("")
}
