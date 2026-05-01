//! Integration tests for `jsonl` cube.
//!
//! Constructor Pattern: one scenario per test. We mount `jsonl.rs` via
//! `#[path]` (same pattern as `integration.rs`) so no library crate
//! surface is required. Fixtures are written to `tempfile::TempDir` —
//! nothing persists after the test.

#[path = "../src/jsonl.rs"]
mod jsonl;

use jsonl::parse_user_lines;
use std::fs;
use tempfile::TempDir;

// ---------------------------------------------------------------
// 1. mixed_shapes — 5 lines: 2 user-string, 1 user-array-blocks,
//    1 assistant, 1 local-command echo. Expect 3 user lines.
// ---------------------------------------------------------------
#[test]
fn mixed_shapes() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("session.jsonl");
    let body = [
        // 1. top-level type=user, content=string
        r#"{"type":"user","role":"user","content":"посмотри генезис опять"}"#,
        // 2. nested message.role=user, content=string
        r#"{"type":"user","message":{"role":"user","content":"нет, делай так — хватит уже"},"timestamp":"2026-04-22T03:14:15Z"}"#,
        // 3. nested message.role=user, content=array of text blocks
        r#"{"type":"user","message":{"role":"user","content":[{"type":"text","text":"стоп. почему ты опять полез не туда?"}]}}"#,
        // 4. assistant — must be ignored
        r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"let me reconsider."}]}}"#,
        // 5. local-command echo — must be filtered
        r#"{"type":"user","role":"user","content":"<local-command-caveat>you ran /effort</local-command-caveat>"}"#,
    ]
    .join("\n");
    fs::write(&path, body).unwrap();

    let out = parse_user_lines(&path).unwrap();
    assert_eq!(out.len(), 3, "expected 3 user lines, got {}: {:#?}", out.len(), out);

    assert!(out[0].text.contains("генезис"), "line 1: {}", out[0].text);
    assert_eq!(out[0].line_no, 1);
    assert!(out[0].timestamp.is_none());

    assert!(out[1].text.contains("хватит"), "line 2: {}", out[1].text);
    assert_eq!(out[1].line_no, 2);
    assert_eq!(
        out[1].timestamp.as_deref(),
        Some("2026-04-22T03:14:15Z"),
        "timestamp passthrough"
    );

    assert!(out[2].text.contains("почему ты опять"), "line 3: {}", out[2].text);
    assert_eq!(out[2].line_no, 3);

    let joined = out.iter().map(|l| l.text.as_str()).collect::<Vec<_>>().join("|");
    assert!(!joined.contains("reconsider"), "assistant leaked: {joined}");
    assert!(!joined.contains("local-command-caveat"), "echo leaked: {joined}");
}

// ---------------------------------------------------------------
// 2. malformed_line_skipped — one bad JSON line in the middle must
//    NOT abort parsing; parser returns what it could extract.
// ---------------------------------------------------------------
#[test]
fn malformed_line_skipped() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("malformed.jsonl");
    let body = [
        r#"{"type":"user","content":"я же уже просил — не трогай это"}"#,
        r#"{this is not valid json at all"#, // line 2 — malformed
        r#"{"type":"user","content":"опять куда ты полез"}"#,
    ]
    .join("\n");
    fs::write(&path, body).unwrap();

    let out = parse_user_lines(&path).unwrap();
    assert_eq!(out.len(), 2, "malformed line must not abort: {out:?}");
    assert!(out[0].text.contains("уже просил"));
    assert_eq!(out[0].line_no, 1);
    assert!(out[1].text.contains("опять"));
    assert_eq!(out[1].line_no, 3, "line_no must reflect true file position");
}

// ---------------------------------------------------------------
// 3. empty_file_yields_empty_vec
// ---------------------------------------------------------------
#[test]
fn empty_file_yields_empty_vec() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("empty.jsonl");
    fs::write(&path, "").unwrap();

    let out = parse_user_lines(&path).unwrap();
    assert!(out.is_empty(), "empty file should yield empty Vec, got {out:?}");
}

// ---------------------------------------------------------------
// 4. assistant_only_yields_empty_vec
// ---------------------------------------------------------------
#[test]
fn assistant_only_yields_empty_vec() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("assistant-only.jsonl");
    let body = [
        r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"hello"}]}}"#,
        r#"{"type":"assistant","message":{"role":"assistant","content":"plain string"}}"#,
        r#"{"type":"user","content":"<system-reminder>ignore me</system-reminder>"}"#,
    ]
    .join("\n");
    fs::write(&path, body).unwrap();

    let out = parse_user_lines(&path).unwrap();
    assert!(
        out.is_empty(),
        "no real user messages → empty Vec, got {out:?}"
    );
}
