//! Platform gate — macOS Apple Silicon ONLY.
//!
//! Constructor Pattern: this cube has ONE responsibility — answer
//! "is this host able to run mlx_lm?". The gate is checked FIRST in every
//! subcommand handler before any subprocess attempt.
//!
//! Detection uses `cfg!()` so the compiled binary carries the answer
//! statically. A linux ARM box and a macOS Intel box BOTH return
//! `supported = false` with the same stable reason string.

use serde::{Deserialize, Serialize};

/// Reason string is exposed to JSON consumers and tests; treat as stable.
pub const REASON_NOT_MACOS: &str = "MLX requires macOS Apple Silicon";
pub const REASON_NOT_AARCH64: &str = "MLX requires macOS Apple Silicon";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SupportStatus {
    pub supported: bool,
    /// `Some(reason)` iff `supported == false`. JSON consumers can assume
    /// the absence/presence inverse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub host_arch: String,
    pub host_os: String,
}

/// Compile-time gate: `macos + aarch64`. Both must be true.
pub fn is_supported() -> SupportStatus {
    let macos = cfg!(target_os = "macos");
    let arm = cfg!(target_arch = "aarch64");
    let host_arch = host_arch_label();
    let host_os = host_os_label();
    if macos && arm {
        SupportStatus { supported: true, reason: None, host_arch, host_os }
    } else {
        SupportStatus {
            supported: false,
            reason: Some(REASON_NOT_MACOS.to_string()),
            host_arch,
            host_os,
        }
    }
}

/// Stable arch label for JSON output.
pub fn host_arch_label() -> String {
    if cfg!(target_arch = "aarch64") {
        "aarch64".into()
    } else if cfg!(target_arch = "x86_64") {
        "x86_64".into()
    } else {
        "other".into()
    }
}

/// Stable OS label for JSON output.
pub fn host_os_label() -> String {
    if cfg!(target_os = "macos") {
        "macos".into()
    } else if cfg!(target_os = "linux") {
        "linux".into()
    } else if cfg!(target_os = "windows") {
        "windows".into()
    } else {
        "other".into()
    }
}
