//! Bootstrap idempotency: calling `bootstrap` twice must produce the same
//! firmware path on disk and the second call must be a no-op (skipped=true).

use kei_frustration_loop::bootstrap::bootstrap;
use std::fs;
use tempfile::TempDir;

fn write_trace(dir: &std::path::Path, name: &str, body: &str) {
    fs::write(dir.join(name), body).unwrap();
}

#[test]
fn second_call_is_a_skip() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    let traces = home.join(".claude/memory/traces");
    fs::create_dir_all(&traces).unwrap();
    write_trace(
        &traces,
        "session-1.jsonl",
        r#"{"type":"user","content":"стоп опять не туда полез"}"#,
    );

    let first = bootstrap("test-user", &traces, home).unwrap();
    assert!(!first.skipped, "first call should NOT be skipped");
    assert!(
        first.firmware_path.contains("test-user.firmware.gz"),
        "firmware_path: {}",
        first.firmware_path
    );
    let p_first = std::path::Path::new(&first.firmware_path);
    assert!(p_first.exists(), "firmware file must exist after first call");
    let mtime_first = fs::metadata(p_first).unwrap().modified().unwrap();

    // Sleep 1.1s so any rewrite would be reflected in mtime.
    std::thread::sleep(std::time::Duration::from_millis(1_100));

    let second = bootstrap("test-user", &traces, home).unwrap();
    assert!(second.skipped, "second call MUST be skipped");
    assert_eq!(second.firmware_path, first.firmware_path);

    // mtime unchanged — no rewrite occurred.
    let mtime_second = fs::metadata(p_first).unwrap().modified().unwrap();
    assert_eq!(
        mtime_first, mtime_second,
        "firmware file must NOT be rewritten on idempotent re-call"
    );
}

#[test]
fn empty_traces_dir_still_produces_firmware() {
    let dir = TempDir::new().unwrap();
    let home = dir.path();
    let traces = home.join(".claude/memory/traces");
    fs::create_dir_all(&traces).unwrap();

    let r = bootstrap("blank-user", &traces, home).unwrap();
    assert!(!r.skipped);
    assert!(std::path::Path::new(&r.firmware_path).exists());
    // No traces ⇒ no hits, but bootstrap still completed.
    assert_eq!(r.scanned_traces, 0);
    assert_eq!(r.initial_hits, 0);
}
