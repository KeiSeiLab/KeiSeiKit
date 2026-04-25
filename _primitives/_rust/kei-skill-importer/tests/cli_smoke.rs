//! CLI exit-code + JSON-output smoke tests.

use std::path::PathBuf;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_kei-skill-importer");

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parse_subcommand_prints_json_and_exits_zero() {
    let out = Command::new(BIN)
        .arg("parse")
        .arg(fixture("cursor-react-rules.mdc"))
        .output()
        .expect("spawn");
    assert!(out.status.success(),
        "exit code: {:?}; stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout not JSON: {e}; got: {stdout}"));
    assert_eq!(parsed["source_format"].as_str(), Some("Cursor"));
    assert!(parsed["description"].as_str().unwrap_or("").contains("React"));
}

#[test]
fn convert_subcommand_emits_summary_json() {
    let tmp = tempfile::tempdir().unwrap();
    let out = Command::new(BIN)
        .arg("convert")
        .arg(fixture("cline-typescript-paths.md"))
        .arg("--output-dir")
        .arg(tmp.path())
        .output()
        .expect("spawn");
    assert!(out.status.success(),
        "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("JSON summary");
    let emitted = parsed["emitted"].as_str().unwrap();
    assert!(matches!(emitted, "atom" | "recipe" | "primitive"),
        "emitted={emitted}");
    let written = parsed["paths"].as_array().expect("paths array");
    assert_eq!(written.len(), 1);
    let p = PathBuf::from(written[0].as_str().unwrap());
    assert!(p.exists(), "expected written file to exist: {}", p.display());
}

#[test]
fn batch_subcommand_processes_fixture_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let out = Command::new(BIN)
        .arg("batch")
        .arg(&fixture_dir)
        .arg("--output-dir")
        .arg(tmp.path())
        .output()
        .expect("spawn");
    assert!(out.status.success(),
        "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<_> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert!(!lines.is_empty(), "batch produced no JSONL lines");
    for line in &lines {
        let parsed: serde_json::Value =
            serde_json::from_str(line).expect("each line is JSON");
        assert!(parsed.get("source").is_some());
        assert!(parsed.get("ok").is_some());
    }
    // Each line should report ok=true for our well-formed fixtures.
    let oks = lines.iter().filter(|l| l.contains("\"ok\":true")).count();
    assert!(oks >= 4, "expected ≥4 successful imports, got {oks}");
}

#[test]
fn parse_with_explicit_format_flag() {
    let out = Command::new(BIN)
        .arg("parse")
        .arg(fixture("kimi-agent-spec.yaml"))
        .arg("--format")
        .arg("kimi")
        .output()
        .expect("spawn");
    assert!(out.status.success(),
        "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"name\": \"coder\""));
}
