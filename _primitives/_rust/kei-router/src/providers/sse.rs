//! Minimal SSE frame parser, shared by all providers.
//!
//! SSE frames are separated by `\n\n`. Each frame may have multiple lines;
//! we only care about the `data: ` line. Returns the JSON payload string per
//! frame (caller decides how to interpret it).

use bytes::Bytes;

pub struct SseParser {
    buf: String,
}

impl Default for SseParser {
    fn default() -> Self {
        Self::new()
    }
}

impl SseParser {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    /// Push bytes; return any complete `data:` payloads (one per completed frame).
    /// `[DONE]` sentinels are returned verbatim — caller filters.
    pub fn push(&mut self, chunk: &Bytes) -> Vec<String> {
        self.buf.push_str(&String::from_utf8_lossy(chunk));
        let mut out = Vec::new();
        while let Some(idx) = self.buf.find("\n\n") {
            let frame: String = self.buf.drain(..idx + 2).collect();
            if let Some(payload) = data_line(&frame) {
                out.push(payload);
            }
        }
        out
    }
}

fn data_line(frame: &str) -> Option<String> {
    let line = frame.lines().find(|l| l.starts_with("data: "))?;
    let payload = line.trim_start_matches("data: ").trim();
    if payload.is_empty() {
        return None;
    }
    Some(payload.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_single_frame() {
        let mut p = SseParser::new();
        let out = p.push(&Bytes::from("data: {\"x\":1}\n\n"));
        assert_eq!(out, vec!["{\"x\":1}".to_string()]);
    }

    #[test]
    fn handles_split_frame() {
        let mut p = SseParser::new();
        assert!(p.push(&Bytes::from("data: {\"x\"")).is_empty());
        assert_eq!(
            p.push(&Bytes::from(":1}\n\n")),
            vec!["{\"x\":1}".to_string()]
        );
    }

    #[test]
    fn skips_no_data_frame() {
        let mut p = SseParser::new();
        let out = p.push(&Bytes::from("event: ping\n\n"));
        assert!(out.is_empty());
    }

    #[test]
    fn returns_done_sentinel() {
        let mut p = SseParser::new();
        let out = p.push(&Bytes::from("data: [DONE]\n\n"));
        assert_eq!(out, vec!["[DONE]".to_string()]);
    }
}
