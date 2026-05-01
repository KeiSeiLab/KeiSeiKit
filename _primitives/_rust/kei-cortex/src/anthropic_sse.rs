//! Incremental SSE parser for Anthropic Messages streaming responses.
//!
//! SSE frames are separated by `\n\n`; a single chunk may contain a partial
//! frame, so `SseParser` buffers across `push` calls and emits every text
//! delta as it completes. Non-text events (`message_start`, `ping`, etc.)
//! are skipped; the parser intentionally only surfaces `text_delta` payloads.

use bytes::Bytes;
use serde::Deserialize;

/// Anthropic content-block-delta payload (only the subfield we care about).
#[derive(Debug, Deserialize)]
struct DeltaEnvelope {
    #[serde(default)]
    delta: Option<Delta>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[serde(rename = "type", default)]
    kind: Option<String>,
    #[serde(default)]
    text: Option<String>,
}

/// Incremental SSE parser — SSE frames are separated by `\n\n`.
///
/// We buffer partial chunks across `push` calls and return every extracted
/// text delta. Non-text events (`message_start`, `ping`, etc.) are skipped.
pub(crate) struct SseParser {
    buf: String,
}

impl SseParser {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    /// Consume a byte chunk, return every text delta completed in this push.
    pub fn push(&mut self, chunk: &Bytes) -> Vec<String> {
        self.buf.push_str(&String::from_utf8_lossy(chunk));
        let mut out = Vec::new();
        while let Some(idx) = self.buf.find("\n\n") {
            let frame: String = self.buf.drain(..idx + 2).collect();
            if let Some(text) = extract_text(&frame) {
                out.push(text);
            }
        }
        out
    }
}

/// Parse a single SSE frame and return the text delta if present.
pub(crate) fn extract_text(frame: &str) -> Option<String> {
    let data_line = frame.lines().find(|l| l.starts_with("data: "))?;
    let json_part = data_line.trim_start_matches("data: ").trim();
    if json_part.is_empty() || json_part == "[DONE]" {
        return None;
    }
    let env: DeltaEnvelope = serde_json::from_str(json_part).ok()?;
    let d = env.delta?;
    if d.kind.as_deref() != Some("text_delta") {
        return None;
    }
    d.text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_text_delta() {
        let frame = "event: content_block_delta\n\
            data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\"hi\"}}\n\n";
        assert_eq!(extract_text(frame), Some("hi".into()));
    }

    #[test]
    fn ignores_non_text_events() {
        let frame = "event: ping\ndata: {\"type\":\"ping\"}\n\n";
        assert_eq!(extract_text(frame), None);
    }

    #[test]
    fn parser_handles_split_frames() {
        let mut p = SseParser::new();
        let part1 = Bytes::from(
            "event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\"ab\"}",
        );
        let part2 = Bytes::from("}\n\n");
        assert!(p.push(&part1).is_empty());
        assert_eq!(p.push(&part2), vec!["ab".to_string()]);
    }
}
