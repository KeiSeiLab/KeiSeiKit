//! `Machine` → `Recommendations` heuristics.
//!
//! Decision tree, all rationale strings preserved in the output:
//!   1. OS != macOS                        → NotViable
//!   2. AppleSilicon + ≥16 GB              → RunsLargeModels (Mlx, LlamaCpp, Ollama)
//!   3. AppleSilicon + ≥8 GB <16 GB        → RunsMidModels   (Mlx, LlamaCpp, Ollama)
//!   4. AppleSilicon + <8 GB                → RunsSmallModelsOnly (LlamaCpp, Ollama)
//!   5. Intel + ≥16 GB                      → RunsMidModels   (LlamaCpp, Ollama)
//!   6. Intel + <16 GB                      → RunsSmallModelsOnly (Ollama)
//!   7. Other                               → NotViable

use crate::profile::{CpuFamily, Machine, OsFamily};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Capability {
    RunsLargeModels,
    RunsMidModels,
    RunsSmallModelsOnly,
    NoLocalInferenceViable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackendTier {
    MlxNative,
    LlamaCpp,
    Ollama,
    NotViable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Recommendations {
    pub capability: Capability,
    pub viable_backends: Vec<BackendTier>,
    pub max_model_b_params: u32,
    pub rationale: Vec<String>,
}

pub fn recommend(m: &Machine) -> Recommendations {
    if m.os.family != OsFamily::Macos {
        return not_viable_for_os(m);
    }
    let gb = m.memory.total_gb();
    match &m.arch.family {
        CpuFamily::AppleSilicon(_) => apple_silicon_tier(gb),
        CpuFamily::IntelX86_64 => intel_tier(gb),
        CpuFamily::Other => not_viable_unknown_arch(),
    }
}

fn not_viable_for_os(m: &Machine) -> Recommendations {
    Recommendations {
        capability: Capability::NoLocalInferenceViable,
        viable_backends: vec![BackendTier::NotViable],
        max_model_b_params: 0,
        rationale: vec![format!(
            "Wave-56 v1 supports macOS only. Detected os.family = {:?}; Linux / other follow-up wave.",
            m.os.family
        )],
    }
}

fn not_viable_unknown_arch() -> Recommendations {
    Recommendations {
        capability: Capability::NoLocalInferenceViable,
        viable_backends: vec![BackendTier::NotViable],
        max_model_b_params: 0,
        rationale: vec!["Unknown CPU family — cannot map to a backend tier.".into()],
    }
}

fn apple_silicon_tier(gb: u64) -> Recommendations {
    if gb >= 16 {
        return apple_large(gb);
    }
    if gb >= 8 {
        return apple_mid(gb);
    }
    apple_small(gb)
}

fn apple_large(gb: u64) -> Recommendations {
    Recommendations {
        capability: Capability::RunsLargeModels,
        viable_backends: vec![BackendTier::MlxNative, BackendTier::LlamaCpp, BackendTier::Ollama],
        max_model_b_params: 13,
        rationale: vec![
            format!("Apple Silicon, {gb} GB unified memory ⇒ MLX / Metal viable."),
            "Max ≈ 13B params (q4) or ≈ 7B (q8) without OOM headroom loss.".into(),
        ],
    }
}

fn apple_mid(gb: u64) -> Recommendations {
    Recommendations {
        capability: Capability::RunsMidModels,
        viable_backends: vec![BackendTier::MlxNative, BackendTier::LlamaCpp, BackendTier::Ollama],
        max_model_b_params: 7,
        rationale: vec![
            format!("Apple Silicon, {gb} GB unified memory."),
            "Max ≈ 7B params (q4); 13B is tight, MLX / Metal still preferred.".into(),
        ],
    }
}

fn apple_small(gb: u64) -> Recommendations {
    Recommendations {
        capability: Capability::RunsSmallModelsOnly,
        viable_backends: vec![BackendTier::LlamaCpp, BackendTier::Ollama],
        max_model_b_params: 3,
        rationale: vec![
            format!("Apple Silicon, only {gb} GB unified memory."),
            "Max ≈ 3-4B params (q4). MLX skipped — too tight for KV cache.".into(),
        ],
    }
}

fn intel_tier(gb: u64) -> Recommendations {
    if gb >= 16 { intel_mid(gb) } else { intel_small(gb) }
}

fn intel_mid(gb: u64) -> Recommendations {
    Recommendations {
        capability: Capability::RunsMidModels,
        viable_backends: vec![BackendTier::LlamaCpp, BackendTier::Ollama],
        max_model_b_params: 7,
        rationale: vec![
            format!("Intel x86_64, {gb} GB RAM ⇒ CPU-only inference (no Metal acceleration)."),
            "Max ≈ 7B q4 (slow); MLX excluded (Apple-Silicon-only).".into(),
        ],
    }
}

fn intel_small(gb: u64) -> Recommendations {
    Recommendations {
        capability: Capability::RunsSmallModelsOnly,
        viable_backends: vec![BackendTier::Ollama],
        max_model_b_params: 3,
        rationale: vec![
            format!("Intel x86_64 with only {gb} GB RAM."),
            "Max ≈ 3B q4 via Ollama (very slow). LlamaCpp omitted: dependency overhead vs. capability.".into(),
        ],
    }
}
