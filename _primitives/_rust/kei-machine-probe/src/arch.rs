//! CPU / arch detection via `sysctl`.
//!
//! Three sysctl reads:
//!   `sysctl -n hw.model`                 → Mac model id (e.g. `Mac14,7`)
//!   `sysctl -n machdep.cpu.brand_string` → marketing string (`Apple M2 Pro`)
//!   `sysctl -n hw.optional.arm64`        → 1 ⇒ Apple Silicon, 0 ⇒ Intel
//!   `sysctl -n hw.ncpu`                  → physical+logical core count
//!
//! Mapping: `family` from the arm64 flag, `variant` parsed from the
//! brand string. Anything we can't classify falls into
//! `AppleVariant::Unknown` rather than panicking.

use crate::profile::{ArchInfo, AppleVariant, CpuFamily};
use crate::runner::Runner;

pub fn detect_arch(runner: &dyn Runner) -> ArchInfo {
    let model_id = runner
        .run("sysctl", &["-n", "hw.model"])
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let brand = runner
        .run("sysctl", &["-n", "machdep.cpu.brand_string"])
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let arm64 = runner
        .run("sysctl", &["-n", "hw.optional.arm64"])
        .ok()
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let cores = runner
        .run("sysctl", &["-n", "hw.ncpu"])
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(0);

    let family = classify_family(&arm64, &brand);
    ArchInfo { family, brand, model_id, cores }
}

fn classify_family(arm64_flag: &str, brand: &str) -> CpuFamily {
    if arm64_flag == "1" {
        return CpuFamily::AppleSilicon(classify_apple_variant(brand));
    }
    if brand.to_ascii_lowercase().contains("intel") {
        return CpuFamily::IntelX86_64;
    }
    CpuFamily::Other
}

/// Match `brand` (e.g. "Apple M2 Pro") to the closest `AppleVariant`.
/// Order matters: longer suffixes (M2 Pro, M2 Max, M2 Ultra) before M2.
fn classify_apple_variant(brand: &str) -> AppleVariant {
    let b = brand.to_ascii_lowercase();
    for (needle, variant) in VARIANT_TABLE {
        if b.contains(needle) {
            return variant.clone();
        }
    }
    AppleVariant::Unknown
}

const VARIANT_TABLE: &[(&str, AppleVariant)] = &[
    ("m4 max", AppleVariant::M4Max),
    ("m4 pro", AppleVariant::M4Pro),
    ("m4", AppleVariant::M4),
    ("m3 max", AppleVariant::M3Max),
    ("m3 pro", AppleVariant::M3Pro),
    ("m3", AppleVariant::M3),
    ("m2 ultra", AppleVariant::M2Ultra),
    ("m2 max", AppleVariant::M2Max),
    ("m2 pro", AppleVariant::M2Pro),
    ("m2", AppleVariant::M2),
    ("m1 ultra", AppleVariant::M1Ultra),
    ("m1 max", AppleVariant::M1Max),
    ("m1 pro", AppleVariant::M1Pro),
    ("m1", AppleVariant::M1),
];
