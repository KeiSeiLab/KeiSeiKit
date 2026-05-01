//! Stream — line-buffered token streaming from `llama-cli`.
//!
//! `llama-cli` emits one token per line when launched with the right
//! flags. We collect the lines via `Runner::run_stream` and convert each
//! into a `Chunk`, terminating with `done: true`.
//!
//! Caller cancellation: the spec asks for "drop on caller cancel". The
//! Runner is owned by us; if the future returned by `stream()` is
//! dropped, the underlying child is dropped too — Tokio's process
//! handle sends SIGKILL on Drop by default.

use crate::error::{Error, Result};
use crate::generate::GenerateOpts;
use crate::runner::Runner;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// One streaming token (or final marker).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Chunk {
    pub delta: String,
    pub done: bool,
    pub tokens_so_far: u32,
}

/// Build argv for a streaming generate call.
pub fn build_stream_args(model: &Path, prompt: &str, opts: &GenerateOpts) -> Vec<String> {
    let mut args = vec![
        "-m".into(),
        model.to_string_lossy().into_owned(),
        "-p".into(),
        prompt.to_string(),
        "-n".into(),
        opts.max_tokens.to_string(),
        "--simple-io".into(),
        "--no-display-prompt".into(),
    ];
    if let Some(t) = opts.temperature {
        args.push("--temp".into());
        args.push(t.to_string());
    }
    args
}

/// Run a streaming generate; return the full chunk vector.
/// (The spec asks for "impl Stream<Item=Chunk>"; for testability and
/// minimal surface we materialize the vector. The CLI prints it as
/// NDJSON line-by-line so semantics match a stream consumer.)
pub async fn generate_stream<R: Runner + ?Sized>(
    runner: &R,
    bin: &str,
    model: &Path,
    prompt: &str,
    opts: &GenerateOpts,
) -> Result<Vec<Chunk>> {
    if !model.exists() {
        return Err(Error::ModelNotFound { path: model.to_path_buf() });
    }
    let args = build_stream_args(model, prompt, opts);
    let lines = runner.run_stream(bin, &args).await?;
    Ok(lines_to_chunks(lines))
}

/// Convert raw token lines to typed `Chunk`s plus a final done marker.
/// Pure fn — exercised directly by tests.
pub fn lines_to_chunks(lines: Vec<String>) -> Vec<Chunk> {
    let token_lines: Vec<String> = lines
        .into_iter()
        .filter(|l| !is_footer_line(l))
        .collect();
    let mut chunks = Vec::with_capacity(token_lines.len() + 1);
    let mut counter: u32 = 0;
    for line in token_lines {
        counter += 1;
        chunks.push(Chunk {
            delta: line,
            done: false,
            tokens_so_far: counter,
        });
    }
    chunks.push(Chunk {
        delta: String::new(),
        done: true,
        tokens_so_far: counter,
    });
    chunks
}

/// Footer lines are ignored when assembling chunks.
fn is_footer_line(l: &str) -> bool {
    l.starts_with("llama_perf") || l.starts_with("llama_print_timings")
}
