//! Streaming generate — line-delimited NDJSON chunks.
//!
//! mlx_lm `--stream` (or its `--stream-format json` flavour) prints one
//! JSON object per generated token: `{"delta": "X", "tokens_so_far": N}`,
//! optionally followed by a final `{"done": true, ...}` marker.
//!
//! Constructor Pattern: this cube parses chunks from already-captured
//! stdout. Live streaming over a pipe is the consumer's job (or a
//! follow-up cube). The parser tolerates non-JSON warning lines (skipped
//! with no error) so mlx_lm logs interleaved with NDJSON do not break us.

use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Chunk {
    /// Token text appended in this step (empty on a `done`-only marker).
    #[serde(default)]
    pub delta: String,
    /// True only on the terminal marker.
    #[serde(default)]
    pub done: bool,
    /// Cumulative token count up to and including this chunk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens_so_far: Option<u32>,
}

/// Parse a multi-line NDJSON stdout into ordered `Chunk`s. Non-JSON
/// lines and blank lines are skipped silently (mlx_lm interleaves
/// progress bars with the JSON stream).
pub fn parse_stream(stdout: &str) -> Result<Vec<Chunk>, Error> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.starts_with('{') {
            continue;
        }
        match serde_json::from_str::<Chunk>(trimmed) {
            Ok(c) => out.push(c),
            Err(e) => return Err(Error::ParseFailed(format!("line `{trimmed}`: {e}"))),
        }
    }
    Ok(out)
}

/// Concatenate every chunk's `delta` into the final text. Convenience
/// for the non-streaming consumer that wants the whole string back.
pub fn concat_chunks(chunks: &[Chunk]) -> String {
    let mut s = String::new();
    for c in chunks {
        s.push_str(&c.delta);
    }
    s
}
