//! Integration tests for ingest of REAL Claude Code trace shape.
//!
//! Wave A (2026-05-01) — verifies the schema-mismatch fix: nested
//! `tool_use` blocks inside `message.content[]` are extracted, one row
//! per block lands in `events` with `tool` populated, `file_path`
//! pulled from `input.file_path`, `cwd` from the top-level field.

use kei_memory::{ingest, schema};
use rusqlite::Connection;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

fn open_tmp() -> (TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("kei-memory.sqlite");
    let conn = Connection::open(&db).unwrap();
    schema::migrate(&conn).unwrap();
    (dir, conn)
}

/// Build a 5-line JSONL fixture mirroring real Claude Code trace shape:
///   1. permission-mode header line (skipped — no message)
///   2. user prompt line (no tool_use, becomes one event row)
///   3. assistant line with TWO tool_use blocks: Bash + Read
///   4. user line with tool_result for the Read (is_error=false)
///   5. user line with tool_result for the Bash (is_error=true)
fn write_real_trace(dir: &TempDir, name: &str) -> std::path::PathBuf {
    let p = dir.path().join(name);
    let mut f = fs::File::create(&p).unwrap();
    let lines = [
        // line 1: permission-mode header — no message, ingested as "other"
        r#"{"type":"permission-mode","permissionMode":"default","sessionId":"sx"}"#,
        // line 2: user prompt
        r#"{"type":"user","timestamp":"2026-05-01T10:00:00Z","sessionId":"sx",
            "cwd":"/work","gitBranch":"main","uuid":"u1",
            "message":{"role":"user","content":"please read /a.rs and run ls"}}"#,
        // line 3: assistant with TWO tool_use blocks
        r#"{"type":"assistant","timestamp":"2026-05-01T10:00:05Z","sessionId":"sx",
            "cwd":"/work","gitBranch":"main","uuid":"u2","parentUuid":"u1",
            "message":{"role":"assistant","content":[
                {"type":"tool_use","id":"tu_1","name":"Bash","input":{"command":"ls"}},
                {"type":"tool_use","id":"tu_2","name":"Read","input":{"file_path":"/a.rs"}}
            ]}}"#,
        // line 4: user tool_result for Read (success)
        r#"{"type":"user","timestamp":"2026-05-01T10:00:06Z","sessionId":"sx",
            "cwd":"/work","gitBranch":"main","uuid":"u3","parentUuid":"u2",
            "message":{"role":"user","content":[
                {"type":"tool_result","tool_use_id":"tu_2","content":"file content","is_error":false}
            ]}}"#,
        // line 5: user tool_result for Bash (error)
        r#"{"type":"user","timestamp":"2026-05-01T10:00:07Z","sessionId":"sx",
            "cwd":"/work","gitBranch":"main","uuid":"u4","parentUuid":"u2",
            "message":{"role":"user","content":[
                {"type":"tool_result","tool_use_id":"tu_1","content":"command not found","is_error":true}
            ]}}"#,
    ];
    for l in &lines {
        // Write each line as a single JSONL record. The fixture uses raw
        // string literals split across source lines for readability;
        // collapse internal newlines so the consumer sees one JSON object
        // per JSONL line.
        let collapsed: String = l
            .lines()
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join("");
        writeln!(f, "{}", collapsed).unwrap();
    }
    p
}

#[test]
fn real_trace_extracts_nested_tool_uses() {
    let (d, conn) = open_tmp();
    let path = write_real_trace(&d, "real.jsonl");
    let n = ingest::ingest_jsonl(&conn, "sx", &path).unwrap();
    // Expected row count:
    //   line 1 (permission-mode) → 1 row
    //   line 2 (user prompt)     → 1 row
    //   line 3 (assistant w/ 2 tool_uses) → 2 rows
    //   line 4 (user tool_result, no tool) → 1 row
    //   line 5 (user tool_result, no tool) → 1 row
    // Total: 6 rows.
    assert_eq!(n, 6, "expected 6 events, got {n}");

    // Distinct tools must include both Bash and Read.
    let tools: Vec<String> = conn
        .prepare("SELECT DISTINCT tool FROM events WHERE tool IS NOT NULL ORDER BY tool")
        .unwrap()
        .query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    assert!(tools.contains(&"Bash".to_string()), "tools must contain Bash, got {tools:?}");
    assert!(tools.contains(&"Read".to_string()), "tools must contain Read, got {tools:?}");

    // file_path must be populated for the Read row.
    let read_fp: Option<String> = conn
        .query_row(
            "SELECT file_path FROM events WHERE tool = 'Read' LIMIT 1",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(read_fp.as_deref(), Some("/a.rs"));

    // cwd must be populated on assistant + user lines.
    let cwd_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM events WHERE cwd = '/work'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(cwd_count >= 4, "cwd populated on >=4 rows, got {cwd_count}");

    // event_class must include both tool_use:Bash and tool_use:Read.
    let classes: Vec<String> = conn
        .prepare("SELECT DISTINCT event_class FROM events ORDER BY event_class")
        .unwrap()
        .query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();
    assert!(classes.iter().any(|c| c == "tool_use:Bash"), "expected tool_use:Bash, got {classes:?}");
    assert!(classes.iter().any(|c| c == "tool_use:Read"), "expected tool_use:Read, got {classes:?}");
}
