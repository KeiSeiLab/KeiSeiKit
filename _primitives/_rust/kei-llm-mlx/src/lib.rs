//! kei-llm-mlx — public library surface (Wave 59).
//!
//! Adapter to Apple MLX inference framework for native Apple Silicon
//! local-LLM hosting. Wraps the canonical `mlx_lm.generate` and
//! `mlx_lm.server` Python CLIs (installed via `pip install mlx-lm`).
//!
//! Position: parallel sibling of `kei-llm-ollama` (W57) and
//! `kei-llm-llamacpp` (W58); glued together by `kei-llm-router` (W60).
//!
//! Design decisions:
//! - Shell-out, NOT PyO3 — keeps the crate Apple-Silicon-only by gate,
//!   not by conditional compilation, and avoids dragging Python build
//!   deps into Rust.
//! - Constructor Pattern — every responsibility (platform gate, binary
//!   discovery, model list, generate, stream parse, server spawn,
//!   error) lives in its own cube ≤200 LOC.
//! - Runner trait — every subprocess goes through `runner::Runner` so
//!   tests substitute `MockRunner` and never invoke real `mlx_lm`.

pub mod cli;
pub mod discovery;
pub mod error;
pub mod generate;
pub mod models;
pub mod platform;
pub mod runner;
pub mod server;
pub mod stream;

pub use discovery::{discover, MlxBins};
pub use error::{exit_code_for, Error};
pub use generate::{generate, GenerateOpts, Response};
pub use models::{classify, default_cache_dir, is_mlx_quantised, list_models, ModelEntry};
pub use platform::{host_arch_label, host_os_label, is_supported, SupportStatus};
pub use runner::{fixture_stem, MockRunner, Runner, RunOutput, SystemRunner};
pub use server::{
    build_spec, build_argv as build_server_argv, is_localhost, openai_compat_url, ServerHandle,
    ServerSpec, DEFAULT_HOST, DEFAULT_PORT,
};
pub use stream::{concat_chunks, parse_stream, Chunk};
