//! Round-trip test for feedback JSONL: append rows, count them, read them
//! back and assert the original structs survived JSON encoding/decoding.

use kei_frustration_loop::feedback::{
    append_feedback, count_pending, read_all, Feedback, Label,
};
use tempfile::TempDir;

fn make_fb(hit: &str, label: Label, msg: &str) -> Feedback {
    Feedback {
        hit_id: hit.to_string(),
        message: msg.to_string(),
        label,
        category: "frustration-tone".to_string(),
        ts: 1_700_000_000,
        user: "test-user".to_string(),
    }
}

#[test]
fn append_count_read_roundtrip() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("test.feedback.jsonl");

    let a = make_fb("hit-1", Label::Correct, "опять не так делаешь");
    let b = make_fb("hit-2", Label::Wrong, "this is fine");
    let c = make_fb(
        "hit-3",
        Label::NewCategory("politeness".to_string()),
        "пожалуйста сделай иначе",
    );

    append_feedback(&path, &a).unwrap();
    append_feedback(&path, &b).unwrap();
    append_feedback(&path, &c).unwrap();

    let n = count_pending(&path).unwrap();
    assert_eq!(n, 3, "expected 3 rows, got {n}");

    let rows = read_all(&path).unwrap();
    assert_eq!(rows.len(), 3, "round-trip lost rows: {rows:?}");
    assert_eq!(rows[0], a);
    assert_eq!(rows[1], b);
    assert_eq!(rows[2], c);
}

#[test]
fn count_on_missing_file_is_zero() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("never-written.jsonl");
    let n = count_pending(&path).unwrap();
    assert_eq!(n, 0, "missing file should yield 0, got {n}");
    let rows = read_all(&path).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn label_parse_correct_wrong_new() {
    assert_eq!(Label::parse("correct").unwrap(), Label::Correct);
    assert_eq!(Label::parse("wrong").unwrap(), Label::Wrong);
    assert_eq!(
        Label::parse("new:bias").unwrap(),
        Label::NewCategory("bias".to_string())
    );
    assert!(Label::parse("garbage").is_err());
    assert!(Label::parse("new:").is_err());
}

#[test]
fn malformed_line_does_not_abort_count() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("mixed.jsonl");
    // Write one good row, one malformed, one good row by hand.
    let good = make_fb("h-A", Label::Correct, "ok");
    append_feedback(&path, &good).unwrap();
    {
        use std::fs::OpenOptions;
        use std::io::Write;
        let mut f = OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        writeln!(f, "{{this is bad json").unwrap();
    }
    let good2 = make_fb("h-B", Label::Wrong, "no");
    append_feedback(&path, &good2).unwrap();

    let n = count_pending(&path).unwrap();
    assert_eq!(n, 2, "malformed line should be skipped, got {n}");
}
