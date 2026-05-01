//! Per-backend health check.
//!
//! Constructor Pattern: ONE responsibility — answer "is this backend up
//! enough to take a request right now?" for each of the three local
//! backends. Every check delegates to the underlying W57/W58/W59 crate;
//! the router never spawns processes itself.
//!
//! - **Ollama** — HTTP probe (`kei_llm_ollama::is_running`).
//! - **llama.cpp** — discovery via `kei_llm_llamacpp::discover` (binary present?).
//! - **MLX** — platform gate first, then discovery.

use serde::{Deserialize, Serialize};

use crate::backend::BackendKind;

/// Outcome of a single backend health probe.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackendHealth {
    pub backend_kind: BackendKind,
    pub available: bool,
    /// Short reason — populated whether `available` is true or false.
    pub reason: String,
}

/// Probe Ollama via the W57 HTTP client (1s default timeout).
pub async fn check_ollama() -> BackendHealth {
    let client = kei_llm_ollama::Client::default();
    let running = kei_llm_ollama::is_running(&client).await;
    BackendHealth {
        backend_kind: BackendKind::Ollama,
        available: running,
        reason: if running {
            format!("daemon reachable at {}", client.base_url())
        } else {
            format!("daemon not reachable at {}", client.base_url())
        },
    }
}

/// Probe llama.cpp via the W58 discovery (binary present on PATH or
/// under `KEI_LLAMA_CPP_DIR`).
pub async fn check_llamacpp() -> BackendHealth {
    let runner = kei_llm_llamacpp::RealRunner;
    let bins = match kei_llm_llamacpp::discover(&runner).await {
        Ok(b) => b,
        Err(e) => {
            return BackendHealth {
                backend_kind: BackendKind::LlamaCpp,
                available: false,
                reason: format!("discovery error: {e}"),
            };
        }
    };
    let avail = bins.any_found();
    BackendHealth {
        backend_kind: BackendKind::LlamaCpp,
        available: avail,
        reason: build_llamacpp_reason(avail, &bins),
    }
}

fn build_llamacpp_reason(avail: bool, bins: &kei_llm_llamacpp::BinPaths) -> String {
    if !avail {
        return "neither llama-cli nor llama-server on PATH".into();
    }
    let parts: Vec<String> = [
        bins.llama_cli.as_ref().map(|p| format!("cli={}", p.display())),
        bins.llama_server.as_ref().map(|p| format!("server={}", p.display())),
    ]
    .into_iter()
    .flatten()
    .collect();
    parts.join(", ")
}

/// Probe MLX — combined platform gate + binary discovery.
pub fn check_mlx() -> BackendHealth {
    let support = kei_llm_mlx::is_supported();
    if !support.supported {
        return BackendHealth {
            backend_kind: BackendKind::Mlx,
            available: false,
            reason: support.reason.unwrap_or_else(|| "unsupported".into()),
        };
    }
    let runner = kei_llm_mlx::SystemRunner;
    let bins = kei_llm_mlx::discover(&runner);
    BackendHealth {
        backend_kind: BackendKind::Mlx,
        available: bins.any_present(),
        reason: build_mlx_reason(&bins),
    }
}

fn build_mlx_reason(bins: &kei_llm_mlx::MlxBins) -> String {
    if !bins.any_present() {
        return "mlx_lm.generate / mlx_lm.server not on PATH".into();
    }
    let parts: Vec<String> = [
        bins.generate.as_ref().map(|p| format!("generate={}", p.display())),
        bins.server.as_ref().map(|p| format!("server={}", p.display())),
    ]
    .into_iter()
    .flatten()
    .collect();
    parts.join(", ")
}

/// Health-check ALL three backends in parallel. Returned in the
/// canonical order (MLX first, Ollama last).
pub async fn check_all() -> Vec<BackendHealth> {
    let mlx = check_mlx();
    let (llama, ollama) = tokio::join!(check_llamacpp(), check_ollama());
    vec![mlx, llama, ollama]
}
