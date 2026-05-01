//! Golden-file test: insert two synthetic agents into an in-memory
//! ledger + memory database, run the export library through its public
//! API, compare emitted JSONL line-by-line against a checked-in
//! fixture.
//!
//! Constructor Pattern: fixture builders live in `tests/common/`.

mod common;

use kei_export_trajectories::{
    normalize_keys, record_to_trajectory, write_jsonl, LedgerReader, Trajectory,
};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

const FROM_TS: i64 = 1_700_000_000;

#[test]
fn export_two_agents_matches_golden() {
    let d = tempdir().unwrap();
    let ledger_p = d.path().join("ledger.sqlite");
    let memory_p = d.path().join("memory.sqlite");
    let repo = d.path().join("repo");
    common::build_ledger(&ledger_p);
    common::build_memory(&memory_p);
    common::build_artefacts(&repo);

    let reader = LedgerReader::new(&ledger_p)
        .with_memory(&memory_p)
        .with_repo_root(&repo);
    let recs = reader.read_since(FROM_TS).unwrap();
    assert_eq!(recs.len(), 2);

    let mut trajs: Vec<Trajectory> = recs
        .iter()
        .enumerate()
        .map(|(i, r)| record_to_trajectory(i as u64, r))
        .collect();
    normalize_keys(&mut trajs);

    let out = d.path().join("out.jsonl");
    write_jsonl(&out, &trajs).unwrap();

    let actual = fs::read_to_string(&out).unwrap();
    let golden_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("golden.jsonl");
    let expected = fs::read_to_string(&golden_path).expect("read golden fixture");
    assert_eq!(
        actual.trim_end(),
        expected.trim_end(),
        "JSONL output drifted from golden fixture"
    );
}

#[test]
fn count_matches_inserted_rows() {
    let d = tempdir().unwrap();
    let ledger_p = d.path().join("ledger.sqlite");
    common::build_ledger(&ledger_p);
    let r = LedgerReader::new(&ledger_p);
    assert_eq!(r.count_since(FROM_TS).unwrap(), 2);
    // Cutoff after agent-b's started_ts (300) AND agent-a's started_ts
    // (100) → 0 rows.
    assert_eq!(r.count_since(1_700_000_350).unwrap(), 0);
}

#[test]
fn missing_memory_db_yields_empty_tool_stats() {
    let d = tempdir().unwrap();
    let ledger_p = d.path().join("ledger.sqlite");
    common::build_ledger(&ledger_p);
    let reader = LedgerReader::new(&ledger_p);
    let recs = reader.read_since(FROM_TS).unwrap();
    for r in &recs {
        assert!(r.tool_events.is_empty());
    }
}
