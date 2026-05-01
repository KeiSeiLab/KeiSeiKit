//! Threshold trigger logic. Populate N-1 feedback rows → assert
//! `should_retrain == false`. Add one more → assert `true`. Then run the
//! actual `auto_train` and verify a new firmware file was written.

use kei_frustration_loop::auto_train::{auto_train, resolve_threshold, should_retrain};
use kei_frustration_loop::feedback::{append_feedback, Feedback, Label};
use std::fs;
use tempfile::TempDir;

fn make_fb(i: usize) -> Feedback {
    Feedback {
        hit_id: format!("hit-{i}"),
        message: format!("опять делаешь не так — повторяю вот так {i}"),
        label: Label::Correct,
        category: "repeat-signal".to_string(),
        ts: 1_700_000_000 + i as u64,
        user: "test-user".to_string(),
    }
}

#[test]
fn threshold_flips_at_exactly_n() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.feedback.jsonl");
    let n = 5usize;

    // N-1 rows ⇒ should_retrain == false.
    for i in 0..(n - 1) {
        append_feedback(&path, &make_fb(i)).unwrap();
    }
    assert!(
        !should_retrain(&path, n).unwrap(),
        "expected false at N-1 rows"
    );

    // Add one more ⇒ should_retrain == true.
    append_feedback(&path, &make_fb(n - 1)).unwrap();
    assert!(
        should_retrain(&path, n).unwrap(),
        "expected true at N rows"
    );
}

#[test]
fn auto_train_under_threshold_is_no_op() {
    let dir = TempDir::new().unwrap();
    let traces = dir.path().join("traces");
    fs::create_dir_all(&traces).unwrap();
    fs::write(traces.join("a.txt"), "hello world").unwrap();
    let fb = dir.path().join("fb.jsonl");
    let out = dir.path().join("user.firmware.gz");

    append_feedback(&fb, &make_fb(0)).unwrap();
    let r = auto_train(&traces, &fb, &out, 10).unwrap();
    assert!(!r.trained, "should not train under threshold");
    assert_eq!(r.feedback_count, 1);
    assert_eq!(r.threshold, 10);
    assert!(!out.exists(), "no firmware should be written");
}

#[test]
fn auto_train_over_threshold_writes_firmware() {
    let dir = TempDir::new().unwrap();
    let traces = dir.path().join("traces");
    fs::create_dir_all(&traces).unwrap();
    fs::write(
        traces.join("seed.txt"),
        "the quick brown fox jumps over the lazy dog repeatedly ",
    )
    .unwrap();
    let fb = dir.path().join("fb.jsonl");
    let out = dir.path().join("user.firmware.gz");

    let n = 3usize;
    for i in 0..n {
        append_feedback(&fb, &make_fb(i)).unwrap();
    }
    let r = auto_train(&traces, &fb, &out, n).unwrap();
    assert!(r.trained, "should train at threshold ({n} rows)");
    assert!(r.corpus_size > 0);
    assert!(out.exists(), "firmware file must be written");
}

// `resolve_threshold_*` assertions are merged into one test because they
// share global env state (`KEI_FRUSTRATION_THRESHOLD`). Splitting them
// across `#[test]` functions causes a parallel-execution race in which
// `cli_wins_over_env` sets the var while `uses_default` reads it — the
// v1 worktree has the same race; we close it here without altering the
// behavioural assertions themselves.
#[test]
fn resolve_threshold_default_env_and_cli_priority() {
    // 1. Default path: env var unset → DEFAULT_THRESHOLD (20).
    std::env::remove_var("KEI_FRUSTRATION_THRESHOLD");
    let t_default = resolve_threshold(None);
    assert_eq!(t_default, 20, "default threshold must be 20");

    // 2. Env var overrides default when CLI is None.
    std::env::set_var("KEI_FRUSTRATION_THRESHOLD", "7");
    let t_env = resolve_threshold(None);
    assert_eq!(t_env, 7, "env var must override default when CLI is None");

    // 3. CLI override beats env override.
    let t_cli = resolve_threshold(Some(99));
    std::env::remove_var("KEI_FRUSTRATION_THRESHOLD");
    assert_eq!(t_cli, 99);
}

#[test]
fn auto_train_returns_threshold_in_report() {
    // Sanity test: `auto_train` echoes the threshold back unchanged in the
    // `TrainReport`, regardless of feedback count or trained flag.
    let dir = TempDir::new().unwrap();
    let traces = dir.path().join("traces");
    fs::create_dir_all(&traces).unwrap();
    let fb = dir.path().join("fb.jsonl");
    let out = dir.path().join("user.firmware.gz");

    let r = auto_train(&traces, &fb, &out, 42).unwrap();
    assert_eq!(r.threshold, 42, "threshold echoed verbatim");
    assert_eq!(r.feedback_count, 0, "no feedback yet");
    assert!(!r.trained);
}
