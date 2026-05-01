//! Inline unit tests for `memory_persist.rs`.
//!
//! Constructor Pattern: extracted to a sibling so the parent stays
//! ≤200 LOC. Tests cover the pure classifier and the on-disk write
//! path against a fresh tempfile sqlite.

use super::*;
use kei_pet::memory::recent;
use tempfile::TempDir;

fn tag() -> MemoryTag {
    MemoryTag {
        user_id: "alice".into(),
        pet_name: "felix".into(),
    }
}

#[test]
fn classify_treats_empty_as_empty() {
    assert_eq!(classify_reply(""), PersistOutcome::Empty);
    assert_eq!(classify_reply("   \n"), PersistOutcome::Empty);
}

#[test]
fn classify_treats_short_circuit() {
    assert_eq!(classify_reply("Nothing to save."), PersistOutcome::NothingToSave);
    assert_eq!(classify_reply("nothing to save"), PersistOutcome::NothingToSave);
}

#[test]
fn classify_marks_error_replies() {
    let r = classify_reply("[memory-review-error] timeout");
    assert!(matches!(r, PersistOutcome::Error(_)));
}

#[test]
fn classify_real_reply_is_write_candidate() {
    let r = classify_reply("Saved a fact about user.");
    assert!(matches!(r, PersistOutcome::Wrote(_)));
}

#[test]
fn record_review_writes_to_fresh_db() {
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("mem.sqlite");
    let outcome = record_review_blocking(&db, &tag(), "Saved a fact about user.");
    let row_id = match outcome {
        PersistOutcome::Wrote(id) => id,
        other => panic!("expected Wrote, got {other:?}"),
    };
    assert!(row_id > 0);

    // Reading back via kei-pet confirms the row exists and is tagged
    // with the review role.
    let conn = rusqlite::Connection::open(&db).unwrap();
    let rows = recent(&conn, &tag(), 10).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].role, REVIEW_ROLE);
    assert_eq!(rows[0].text, "Saved a fact about user.");
}

#[test]
fn record_review_skips_short_circuit() {
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("mem.sqlite");
    let outcome = record_review_blocking(&db, &tag(), "Nothing to save.");
    assert_eq!(outcome, PersistOutcome::NothingToSave);
    // No row was written even though we passed a non-empty reply.
    if !db.exists() {
        return; // schema never created — definitely no rows
    }
    let conn = rusqlite::Connection::open(&db).unwrap();
    ensure_schema(&conn).unwrap();
    let rows = recent(&conn, &tag(), 10).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn record_review_skips_error_reply() {
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("mem.sqlite");
    let outcome = record_review_blocking(&db, &tag(), "[memory-review-error] boom");
    assert!(matches!(outcome, PersistOutcome::Error(_)));
}

#[test]
fn persist_request_runs_through_struct_path() {
    let tmp = TempDir::new().unwrap();
    let db = tmp.path().join("mem.sqlite");
    let req = PersistRequest {
        db_path: db.clone(),
        tag: tag(),
        reply: "User prefers Rust.".into(),
    };
    let out = req.run();
    assert!(matches!(out, PersistOutcome::Wrote(_)));
}
