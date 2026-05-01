//! JSONL session transcript parser — extract USER messages only.
//!
//! Raw Claude Code session files (`~/.claude/projects/*/sessions/*.jsonl`)
//! are newline-delimited JSON. One message per line. Shapes vary across
//! Claude Code versions — see `extract_user_text` for the five known
//! variants we normalise.
//!
//! Constructor Pattern: one file, one public entry (`parse_user_lines`).
//! Helpers are small and private. No full-file `read_to_string` — we
//! stream via `BufReader::lines()` so a 1.4 GB corpus never materialises
//! in memory all at once.
//!
//! System echoes (`<local-command-*>`, `<command-*>`, `<system-reminder>`,
//! `<task-notification>`, `<command-stderr>`) are injected by the CLI
//! runtime, not typed by the user — we drop them here, not in the
//! category regexes.

use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// One user-written message extracted from a raw `.jsonl` session file.
///
/// `line_no` is 1-based so reviewers can open the file in their editor
/// and jump directly to the hit. `timestamp` is the ISO 8601 string the
/// runtime wrote into the `.timestamp` field, if present — we do not
/// parse or reformat it (keeps the cube pure).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonlUserLine {
    pub file: PathBuf,
    pub line_no: usize,
    pub timestamp: Option<String>,
    pub text: String,
}

/// Stream-parse a `.jsonl` session file, yielding user messages only.
///
/// Malformed JSON lines are silently skipped — one bad line must not
/// abort parsing of a 100 MB session. Non-user messages (assistant,
/// tool-result, meta) are also skipped. System-echo user messages
/// (local-command, system-reminder) are dropped here.
pub fn parse_user_lines(path: &Path) -> Result<Vec<JsonlUserLine>> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let Ok(raw) = line else { continue };
        if raw.trim().is_empty() {
            continue;
        }
        if let Some(entry) = parse_one_line(path, idx + 1, &raw) {
            out.push(entry);
        }
    }
    Ok(out)
}

/// Parse a single JSONL line; return `None` unless it is a real user
/// message with non-echo content.
fn parse_one_line(path: &Path, line_no: usize, raw: &str) -> Option<JsonlUserLine> {
    let v: Value = serde_json::from_str(raw).ok()?;
    if !is_user_message(&v) {
        return None;
    }
    let text = extract_user_text(&v)?;
    let trimmed = text.trim();
    if trimmed.is_empty() || is_system_echo(trimmed) {
        return None;
    }
    Some(JsonlUserLine {
        file: path.to_path_buf(),
        line_no,
        timestamp: v.get("timestamp").and_then(|t| t.as_str()).map(str::to_owned),
        text: trimmed.to_owned(),
    })
}

/// A record is a user message if either the top-level `type == "user"` OR
/// the nested `.message.role == "user"`. Both shapes appear in real data.
fn is_user_message(v: &Value) -> bool {
    if v.get("type").and_then(Value::as_str) == Some("user") {
        return true;
    }
    v.get("message")
        .and_then(|m| m.get("role"))
        .and_then(Value::as_str)
        == Some("user")
}

/// Extract visible text from the three known content shapes:
/// (1) `.content` as string,
/// (2) `.message.content` as string,
/// (3) `.message.content` as array of blocks with `type: "text"`.
fn extract_user_text(v: &Value) -> Option<String> {
    if let Some(s) = v.get("content").and_then(Value::as_str) {
        return Some(s.to_owned());
    }
    let inner = v.get("message").and_then(|m| m.get("content"))?;
    if let Some(s) = inner.as_str() {
        return Some(s.to_owned());
    }
    inner.as_array().map(|a| join_text_blocks(a))
}

/// Concatenate all `{type: "text", text: "..."}` blocks in an array.
/// Non-text blocks (tool_use, image, etc.) are skipped — they are not
/// typed prose and won't match any frustration regex meaningfully.
fn join_text_blocks(blocks: &[Value]) -> String {
    let mut buf = String::new();
    for b in blocks {
        if b.get("type").and_then(Value::as_str) != Some("text") {
            continue;
        }
        if let Some(t) = b.get("text").and_then(Value::as_str) {
            if !buf.is_empty() {
                buf.push('\n');
            }
            buf.push_str(t);
        }
    }
    buf
}

/// True for CLI-injected tags that masquerade as user turns. These are
/// emitted by Claude Code itself (slash-command echo, stdout capture,
/// system reminder) — the human never typed them.
fn is_system_echo(s: &str) -> bool {
    const MARKERS: &[&str] = &[
        "<local-command-",
        "<command-",
        "<system-reminder>",
        "<task-notification>",
    ];
    MARKERS.iter().any(|m| s.starts_with(m))
}
