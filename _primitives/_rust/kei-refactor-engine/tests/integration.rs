//! Integration tests for kei-refactor-engine.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kei-refactor-engine"))
}

fn sample_json(extra_manual: bool) -> String {
    let mut items = vec![serde_json::json!({
        "category": "blocks",
        "severity": "medium",
        "files": ["_blocks/a.md", "_blocks/b.md"],
        "evidence": "shingle-Jaccard 72% overlap",
        "suggested_fix": "keep better-cited",
        "auto_resolvable": true
    })];
    if extra_manual {
        items.push(serde_json::json!({
            "category": "rules",
            "severity": "high",
            "files": ["rules/x.md", "rules/y.md"],
            "evidence": "contradictory directive on 'push'",
            "suggested_fix": "review both",
            "auto_resolvable": false
        }));
    }
    serde_json::json!({ "hit_count": items.len(), "conflicts": items }).to_string()
}

#[test]
fn plan_only_prints_markdown() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("c.json");
    fs::write(&input, sample_json(false)).unwrap();
    let out = std::process::Command::new(bin())
        .args(["--input"])
        .arg(&input)
        .output()
        .unwrap();
    assert!(out.status.success());
    let md = String::from_utf8(out.stdout).unwrap();
    assert!(md.contains("# Deep-sleep refactor plan"));
    assert!(md.contains("Auto-apply"));
}

#[test]
fn manual_items_listed_but_not_in_autoresolve() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("c.json");
    let plan_out = tmp.path().join("plan.md");
    let patch_out = tmp.path().join("plan-autoresolve.md");
    fs::write(&input, sample_json(true)).unwrap();
    let out = std::process::Command::new(bin())
        .args(["--input"])
        .arg(&input)
        .args(["--apply-to-branch", "deep-sleep/test", "--plan-out"])
        .arg(&plan_out)
        .args(["--patch-out"])
        .arg(&patch_out)
        .output()
        .unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
    let md = fs::read_to_string(&plan_out).unwrap();
    assert!(md.contains("Requires human decision"));
    let autoresolve = fs::read_to_string(&patch_out).unwrap();
    // autoresolve must NOT reference rules/x.md from the manual item
    assert!(!autoresolve.contains("rules/x.md"), "autoresolve leaked manual item: {}", autoresolve);
    assert!(autoresolve.contains("_blocks/a.md"));
    // And it must NOT claim to be a unified diff.
    assert!(!autoresolve.contains("--- a/"));
    assert!(!autoresolve.contains("+++ b/"));
}

#[test]
fn empty_conflicts_produce_valid_plan() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("c.json");
    fs::write(&input, r#"{"hit_count": 0, "conflicts": []}"#).unwrap();
    let out = std::process::Command::new(bin())
        .args(["--input"])
        .arg(&input)
        .output()
        .unwrap();
    assert!(out.status.success());
    let md = String::from_utf8(out.stdout).unwrap();
    assert!(md.contains("Total conflicts: **0**"));
}

#[test]
fn stdin_input_works() {
    let tmp = TempDir::new().unwrap();
    let _ = tmp; // kept for parity
    let mut child = std::process::Command::new(bin())
        .args(["--input", "-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(sample_json(false).as_bytes()).unwrap();
    }
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout).unwrap().contains("refactor plan"));
}

#[test]
fn autoresolve_header_shows_counts() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("c.json");
    let patch_out = tmp.path().join("plan-autoresolve.md");
    fs::write(&input, sample_json(true)).unwrap();
    std::process::Command::new(bin())
        .args(["--input"])
        .arg(&input)
        .args(["--apply-to-branch", "deep-sleep/a", "--patch-out"])
        .arg(&patch_out)
        .output()
        .unwrap();
    let autoresolve = fs::read_to_string(&patch_out).unwrap();
    assert!(autoresolve.contains("Auto-apply candidates: 1"));
    assert!(autoresolve.contains("Human-decision items"));
    // Retraction check: no unified-diff headers.
    assert!(!autoresolve.contains("--- a/"));
    assert!(!autoresolve.contains("+++ b/"));
}
