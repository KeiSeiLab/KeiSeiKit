use super::{path_resolve, regex_match};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

/// Walk dotted path (no wildcards) into JSON value.
fn walk<'a>(v: &'a Value, dotted: &str) -> Option<&'a Value> {
    let mut cur = v;
    for seg in dotted.split('.') {
        if seg.is_empty() {
            return None;
        }
        cur = cur.as_object()?.get(seg)?;
    }
    Some(cur)
}

/// Stringify the leaf JSON value for comparison.
/// String -> raw text; numbers/bools -> their JSON form; objects/arrays -> reject.
fn stringify_leaf(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => Some("null".to_string()),
        Value::Object(_) | Value::Array(_) => None,
    }
}

/// Security fix E: when an actual JSON value mismatches the expected scalar,
/// the original implementation echoed the actual value verbatim into the
/// FAIL message. For sensitive fields (tokens, hashed passwords, keys) this
/// leaks the value into CI logs / stderr. Replace the echo with a non-
/// reversible fingerprint: byte length + first 4 bytes of sha256.
fn redact(value: &str) -> String {
    let mut h = Sha256::new();
    h.update(value.as_bytes());
    let digest = h.finalize();
    format!(
        "len={} sha256[:8]={:02x}{:02x}{:02x}{:02x}",
        value.len(),
        digest[0],
        digest[1],
        digest[2],
        digest[3]
    )
}

pub fn check(file: &Path, dotted: &str, expected: &str, root: &Path) -> (bool, String) {
    let resolved = match path_resolve::resolve_confined(file, root) {
        Ok(p) => p,
        Err(e) => return (false, e),
    };
    let contents = match regex_match::read_capped(&resolved) {
        Ok(s) => s,
        Err(e) => return (false, e),
    };
    let parsed: Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(e) => return (false, format!("invalid JSON in {}: {}", resolved.display(), e)),
    };
    let leaf = match walk(&parsed, dotted) {
        Some(v) => v,
        None => return (false, format!("path `{}` not found in {}", dotted, resolved.display())),
    };
    let actual = match stringify_leaf(leaf) {
        Some(s) => s,
        None => return (false, format!("path `{}` is object/array (not scalar)", dotted)),
    };
    if actual == expected {
        (true, String::new())
    } else {
        (false, mismatch_msg(&resolved, dotted, &actual))
    }
}

fn mismatch_msg(file: &Path, dotted: &str, actual: &str) -> String {
    format!("json `{}`.{} mismatch (got {})", file.display(), dotted, redact(actual))
}
