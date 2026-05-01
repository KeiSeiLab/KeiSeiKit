//! RAM detection via `sysctl hw.memsize` + `vm_stat`.
//!
//! `sysctl -n hw.memsize` → total RAM in bytes (always reliable).
//! `vm_stat` → per-page activity. Apple's standard page size is 16 KiB on
//! Apple Silicon, 4 KiB on Intel; we read the first line ("page size of
//! N bytes") and use it. `available_bytes` ≈ (free + inactive +
//! speculative + purgeable) × page_size. `pressure_pct` is the share of
//! total occupied by wired+active+compressed.

use crate::profile::MemoryInfo;
use crate::runner::Runner;
use regex::Regex;

pub fn detect_memory(runner: &dyn Runner) -> MemoryInfo {
    let total_bytes = runner
        .run("sysctl", &["-n", "hw.memsize"])
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(0);
    let vm_stat = runner.run("vm_stat", &[]).unwrap_or_default();
    let (available_bytes, pressure_pct) = parse_vm_stat(&vm_stat, total_bytes);
    MemoryInfo { total_bytes, available_bytes, pressure_pct }
}

fn parse_vm_stat(text: &str, total_bytes: u64) -> (u64, u32) {
    let page_size = parse_page_size(text).unwrap_or(16 * 1024);
    let counts = ParsedCounts::from_vm_stat(text);
    let free_pages = counts.free + counts.inactive + counts.speculative + counts.purgeable;
    let available_bytes = free_pages.saturating_mul(page_size);
    let used_pages = counts.wired + counts.active + counts.compressed;
    let pressure_pct = if total_bytes == 0 {
        0
    } else {
        let used_bytes = used_pages.saturating_mul(page_size);
        ((used_bytes as f64 / total_bytes as f64) * 100.0).round() as u32
    };
    (available_bytes, pressure_pct)
}

fn parse_page_size(text: &str) -> Option<u64> {
    let re = Regex::new(r"page size of (\d+) bytes").ok()?;
    re.captures(text)?.get(1)?.as_str().parse::<u64>().ok()
}

#[derive(Default)]
struct ParsedCounts {
    free: u64,
    inactive: u64,
    speculative: u64,
    purgeable: u64,
    wired: u64,
    active: u64,
    compressed: u64,
}

impl ParsedCounts {
    fn from_vm_stat(text: &str) -> Self {
        let mut p = Self::default();
        for line in text.lines() {
            if let Some(n) = parse_pages_line(line, "Pages free") {
                p.free = n;
            } else if let Some(n) = parse_pages_line(line, "Pages inactive") {
                p.inactive = n;
            } else if let Some(n) = parse_pages_line(line, "Pages speculative") {
                p.speculative = n;
            } else if let Some(n) = parse_pages_line(line, "Pages purgeable") {
                p.purgeable = n;
            } else if let Some(n) = parse_pages_line(line, "Pages wired down") {
                p.wired = n;
            } else if let Some(n) = parse_pages_line(line, "Pages active") {
                p.active = n;
            } else if let Some(n) = parse_pages_line(line, "Pages occupied by compressor") {
                p.compressed = n;
            }
        }
        p
    }
}

/// Parse a vm_stat line like `"Pages free:    123456."` → 123456.
fn parse_pages_line(line: &str, prefix: &str) -> Option<u64> {
    let rest = line.strip_prefix(prefix)?.trim_start_matches(':').trim();
    rest.trim_end_matches('.').replace(',', "").parse::<u64>().ok()
}
