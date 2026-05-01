//! Nightly-scan smoke test. Build a tiny traces dir with three JSONL files,
//! one of which contains an `опять` user line that the regex SSoT
//! `repeat-signal` category must match. Assert ScanReport.hits >= 1 and
//! the queue file has at least one row.

use frustration_matrix::firmware::Firmware;
use kei_frustration_loop::nightly::{nightly_scan, QueuedHit};
use std::fs;
use tempfile::TempDir;

fn write_trace(dir: &std::path::Path, name: &str, lines: &[&str]) {
    let path = dir.join(name);
    let body = lines.join("\n");
    fs::write(&path, body).unwrap();
}

#[test]
fn three_files_one_hit() {
    let dir = TempDir::new().unwrap();
    let traces = dir.path().join("traces");
    fs::create_dir_all(&traces).unwrap();
    let queue = dir.path().join("queue.jsonl");

    // File 1 — assistant only, must produce zero hits.
    write_trace(
        &traces,
        "session-a.jsonl",
        &[r#"{"type":"assistant","message":{"role":"assistant","content":"hello"}}"#],
    );
    // File 2 — user message that matches the `repeat-signal` regex
    // (`\bопять\b` is in `categories.rs`). One hit expected.
    write_trace(
        &traces,
        "session-b.jsonl",
        &[
            r#"{"type":"user","content":"стоп — ты опять полез не туда"}"#,
        ],
    );
    // File 3 — user message that's too short / no triggers, zero hits.
    write_trace(
        &traces,
        "session-c.jsonl",
        &[r#"{"type":"user","content":"ok thanks for the help"}"#],
    );

    // Firmware can be a trivial 4-char model — nightly_scan does not
    // currently use the firmware for the regex path; we still must pass it.
    let firmware = Firmware::train_from_text("hello world ", 1);

    let report = nightly_scan(&traces, &firmware, "test-user", 0, &queue).unwrap();

    assert_eq!(report.scanned, 3, "expected 3 files scanned, got {}", report.scanned);
    assert!(report.hits >= 1, "expected ≥1 hit, got {}", report.hits);
    assert!(
        report.by_category.contains_key("repeat-signal"),
        "expected repeat-signal in by_category, got {:?}",
        report.by_category
    );

    // Queue file must exist + contain at least one parseable JSON row.
    let body = fs::read_to_string(&queue).unwrap();
    let mut count = 0usize;
    for line in body.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let _: QueuedHit = serde_json::from_str(line)
            .unwrap_or_else(|e| panic!("queue line not parseable: {e}: {line}"));
        count += 1;
    }
    assert!(count >= 1, "expected ≥1 queued hit, got {count}");
}

#[test]
fn since_filter_excludes_old_traces() {
    let dir = TempDir::new().unwrap();
    let traces = dir.path().join("traces");
    fs::create_dir_all(&traces).unwrap();
    let queue = dir.path().join("queue.jsonl");

    write_trace(
        &traces,
        "old.jsonl",
        &[r#"{"type":"user","content":"опять опять опять"}"#],
    );

    let firmware = Firmware::train_from_text("ab ab ab ", 1);
    // Using a since_ts in the far future — every file's mtime is in the
    // past, so collect_recent_traces should return nothing.
    let report = nightly_scan(
        &traces,
        &firmware,
        "u",
        u64::MAX / 2,
        &queue,
    )
    .unwrap();
    assert_eq!(report.scanned, 0);
    assert_eq!(report.hits, 0);
}
