//! Machine struct — the typed snapshot that `probe` emits.
//!
//! Constructor Pattern: this cube owns the schema. Detectors fill in
//! their respective sub-struct; `recommend()` and `markdown()` consume
//! the whole thing read-only.

use serde::{Deserialize, Serialize};

/// Top-level snapshot. `source_commands` records every shell-out the
/// probe performed so the JSON is self-explaining.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Machine {
    pub os: OsInfo,
    pub arch: ArchInfo,
    pub memory: MemoryInfo,
    pub gpu: GpuInfo,
    pub tooling: ToolingInfo,
    pub source_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OsInfo {
    pub family: OsFamily,
    pub version: String,
    pub build: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OsFamily {
    Macos,
    Linux,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchInfo {
    pub family: CpuFamily,
    pub brand: String,
    pub model_id: String,
    pub cores: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", content = "variant")]
pub enum CpuFamily {
    AppleSilicon(AppleVariant),
    IntelX86_64,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppleVariant {
    M1,
    M1Pro,
    M1Max,
    M1Ultra,
    M2,
    M2Pro,
    M2Max,
    M2Ultra,
    M3,
    M3Pro,
    M3Max,
    M4,
    M4Pro,
    M4Max,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub pressure_pct: u32,
}

impl MemoryInfo {
    pub fn total_gb(&self) -> u64 {
        self.total_bytes / 1024 / 1024 / 1024
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum GpuInfo {
    AppleIntegrated { cores: u32, name: String },
    IntelIntegrated { name: String },
    Discrete { name: String },
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ToolingInfo {
    pub ollama: Option<String>,
    pub homebrew: Option<String>,
    pub llama_cpp: Option<String>,
}
