//! kei-llm-ollama — HTTP adapter for the Ollama daemon (`localhost:11434`).
//!
//! Wave 57 of the local-LLM stack. Ollama is the EXISTING tool — this crate
//! wraps its HTTP API (does not reinvent inference). See README in `docs/api.md`
//! upstream: <https://github.com/ollama/ollama/blob/main/docs/api.md>.

pub mod api;
pub mod cli;
pub mod client;
pub mod error;
pub mod handlers;
pub mod health;
pub mod http_io;
pub mod stream;

pub use api::{
    build_options, ChatReq, ChatResp, GenerateReq, GenerateResp, Message, ModelEntry, PullResp,
    TagsResp, VersionResp,
};
pub use client::{Client, DEFAULT_BASE_URL, DEFAULT_TIMEOUT};
pub use error::ApiError;
pub use health::{is_running, snapshot, HealthSnapshot};
pub use stream::{Chunk, NdjsonBuffer};
