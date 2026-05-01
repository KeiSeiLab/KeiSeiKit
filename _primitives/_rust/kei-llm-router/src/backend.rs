//! `Backend` enum + `BackendKind` tag.
//!
//! Constructor Pattern: ONE responsibility — represent the three concrete
//! local-LLM execution targets the router can pick.
//!
//! - `Mlx` — Apple Silicon native (mlx_lm shell-out).
//! - `LlamaCpp` — local llama.cpp binary + .gguf file on disk.
//! - `Ollama` — Ollama daemon at `127.0.0.1:11434`.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Concrete backend selection — each variant carries the data the
/// downstream caller needs to actually invoke the backend.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Backend {
    Mlx { model_id: String },
    LlamaCpp { gguf_path: PathBuf, model_name: String },
    Ollama { model_tag: String },
}

impl Backend {
    /// Strip variant data, return just the kind tag.
    pub fn kind(&self) -> BackendKind {
        match self {
            Backend::Mlx { .. } => BackendKind::Mlx,
            Backend::LlamaCpp { .. } => BackendKind::LlamaCpp,
            Backend::Ollama { .. } => BackendKind::Ollama,
        }
    }

    /// Identifier used by the caller (model id, tag, or filename).
    pub fn identifier(&self) -> &str {
        match self {
            Backend::Mlx { model_id } => model_id.as_str(),
            Backend::LlamaCpp { model_name, .. } => model_name.as_str(),
            Backend::Ollama { model_tag } => model_tag.as_str(),
        }
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.kind(), self.identifier())
    }
}

/// Variant tag without payload — used in discovery output, health
/// reports, and the `which` subcommand response.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum BackendKind {
    Mlx,
    LlamaCpp,
    Ollama,
}

impl BackendKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            BackendKind::Mlx => "mlx",
            BackendKind::LlamaCpp => "llamacpp",
            BackendKind::Ollama => "ollama",
        }
    }
}

impl fmt::Display for BackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Map a `kei_machine_probe::BackendTier` to the router's `BackendKind`.
/// `NotViable` collapses to `None` (caller filters those out).
pub fn from_tier(t: &kei_machine_probe::BackendTier) -> Option<BackendKind> {
    match t {
        kei_machine_probe::BackendTier::MlxNative => Some(BackendKind::Mlx),
        kei_machine_probe::BackendTier::LlamaCpp => Some(BackendKind::LlamaCpp),
        kei_machine_probe::BackendTier::Ollama => Some(BackendKind::Ollama),
        kei_machine_probe::BackendTier::NotViable => None,
    }
}
