//! NDJSON stream consumer for `/api/generate` and `/api/chat` (`stream: true`).
//!
//! Ollama emits one JSON object per line, terminated by an object with `done: true`.
//! Schema source: <https://github.com/ollama/ollama/blob/main/docs/api.md>

use bytes::Bytes;
use futures::stream::{Stream, StreamExt};
use serde::Deserialize;

use crate::error::ApiError;

/// One streamed chunk from `/api/generate` or `/api/chat`.
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub delta: String,
    pub done: bool,
    pub eval_count: Option<u64>,
    pub eval_duration_ns: Option<u64>,
}

/// Buffer that splits a byte stream into newline-delimited JSON payloads.
pub struct NdjsonBuffer {
    buf: String,
}

impl Default for NdjsonBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl NdjsonBuffer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    /// Push bytes; return any complete JSON lines (one per finished line).
    pub fn push(&mut self, chunk: &Bytes) -> Vec<String> {
        self.buf.push_str(&String::from_utf8_lossy(chunk));
        let mut out = Vec::new();
        while let Some(idx) = self.buf.find('\n') {
            let line: String = self.buf.drain(..idx + 1).collect();
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                out.push(trimmed.to_string());
            }
        }
        out
    }
}

/// Decode one NDJSON line into either a generate-style or chat-style chunk.
pub fn decode_line(line: &str) -> Result<Chunk, ApiError> {
    let env: ChunkEnvelope =
        serde_json::from_str(line).map_err(|e| ApiError::DecodeError(e.to_string()))?;
    let delta = if let Some(msg) = env.message {
        msg.content
    } else {
        env.response.unwrap_or_default()
    };
    Ok(Chunk {
        delta,
        done: env.done,
        eval_count: env.eval_count,
        eval_duration_ns: env.eval_duration,
    })
}

#[derive(Debug, Deserialize)]
struct ChunkEnvelope {
    #[serde(default)]
    response: Option<String>,
    #[serde(default)]
    message: Option<MessageField>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    eval_count: Option<u64>,
    #[serde(default)]
    eval_duration: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct MessageField {
    #[serde(default)]
    content: String,
}

/// Convert a raw bytes-stream (from reqwest) into a stream of [`Chunk`].
pub fn chunk_stream<S>(byte_stream: S) -> impl Stream<Item = Result<Chunk, ApiError>> + Send + 'static
where
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Send + 'static,
{
    let mut buf = NdjsonBuffer::new();
    byte_stream
        .map(move |item| -> Vec<Result<Chunk, ApiError>> {
            match item {
                Ok(bytes) => buf
                    .push(&bytes)
                    .into_iter()
                    .map(|line| decode_line(&line))
                    .collect(),
                Err(e) => vec![Err(ApiError::Transport(e.to_string()))],
            }
        })
        .flat_map(futures::stream::iter)
}
