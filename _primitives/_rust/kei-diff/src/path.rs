//! RFC 6901 JSON Pointer path builder.
//!
//! Root is `""`. Segments join with `/`. Inside a segment, `~` encodes as
//! `~0` and `/` encodes as `~1`. Order matters: `~` must be escaped first
//! when encoding, and `~1` must be decoded before `~0`.

/// Incremental pointer builder. Use `push`/`pop` during recursive traversal;
/// `as_str` yields the current RFC 6901 pointer.
#[derive(Debug, Default, Clone)]
pub struct PathBuf {
    segments: Vec<String>, // already-encoded segments (no leading '/')
}

impl PathBuf {
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    /// Push an object key. Performs RFC 6901 escaping.
    pub fn push_key(&mut self, key: &str) {
        self.segments.push(encode_segment(key));
    }

    /// Push an array index. Always emitted as decimal digits.
    pub fn push_index(&mut self, idx: usize) {
        self.segments.push(idx.to_string());
    }

    pub fn pop(&mut self) {
        self.segments.pop();
    }

    /// Current pointer as a String. Empty string if at root.
    pub fn as_string(&self) -> String {
        if self.segments.is_empty() {
            return String::new();
        }
        let mut out = String::with_capacity(self.segments.iter().map(|s| s.len() + 1).sum());
        for seg in &self.segments {
            out.push('/');
            out.push_str(seg);
        }
        out
    }
}

fn encode_segment(raw: &str) -> String {
    // ~ must be escaped BEFORE / so we don't double-encode.
    raw.replace('~', "~0").replace('/', "~1")
}

/// Parse an RFC 6901 pointer into decoded segments. `""` → `[]`.
/// Returns `None` if pointer is malformed (e.g. doesn't start with `/`
/// and is non-empty).
pub fn parse_pointer(ptr: &str) -> Option<Vec<String>> {
    if ptr.is_empty() {
        return Some(Vec::new());
    }
    if !ptr.starts_with('/') {
        return None;
    }
    let segs = ptr[1..]
        .split('/')
        .map(decode_segment)
        .collect::<Vec<_>>();
    Some(segs)
}

fn decode_segment(raw: &str) -> String {
    // ~1 must be decoded BEFORE ~0 per RFC 6901.
    raw.replace("~1", "/").replace("~0", "~")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_is_empty_string() {
        let p = PathBuf::new();
        assert_eq!(p.as_string(), "");
    }

    #[test]
    fn escape_tilde_and_slash() {
        let mut p = PathBuf::new();
        p.push_key("a/b");
        p.push_key("c~d");
        assert_eq!(p.as_string(), "/a~1b/c~0d");
    }

    #[test]
    fn roundtrip_encode_decode() {
        let mut p = PathBuf::new();
        p.push_key("weird~/key");
        let s = p.as_string();
        let decoded = parse_pointer(&s).unwrap();
        assert_eq!(decoded, vec!["weird~/key".to_string()]);
    }

    #[test]
    fn parse_rejects_malformed() {
        assert!(parse_pointer("no-leading-slash").is_none());
    }
}
