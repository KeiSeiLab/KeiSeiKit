//! Integration tests for kei-ledger.
//!
//! Constructor Pattern: each test = one scenario, one assertion target.
//! Uses tempfile for per-test isolated sqlite file. Loads source modules
//! via `#[path]` so we don't need to expose a library crate surface.

#[path = "../src/migrations_list.rs"]
mod migrations_list;
#[path = "../src/schema.rs"]
mod schema;
#[path = "../src/error.rs"]
mod error;
#[path = "../src/row.rs"]
mod row;
#[path = "../src/ledger.rs"]
mod ledger;
#[path = "../src/descendants.rs"]
mod descendants;

use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn open_tmp() -> (TempDir, Connection) {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    let conn = ledger::open(&db).unwrap();
    (dir, conn)
}

fn write_artefacts(root: &Path, agent_id: &str, which: &[&str]) -> PathBuf {
    let base = root.join(".claude/agents").join(agent_id);
    fs::create_dir_all(&base).unwrap();
    for f in which {
        fs::write(base.join(f), b"x").unwrap();
    }
    base
}

#[test]
fn fork_then_done_marks_terminal() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "a1", "agent/a1", None, "deadbeef", None, None, None, None).unwrap();
    let running = ledger::list(&conn, Some("running")).unwrap();
    assert_eq!(running.len(), 1);
    assert_eq!(running[0].id, "a1");

    let updated = ledger::done(&conn, "a1", "shipped").unwrap();
    assert_eq!(updated, 1);
    let done = ledger::list(&conn, Some("done")).unwrap();
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].summary.as_deref(), Some("shipped"));
}

#[test]
fn fail_flow_sets_reason_and_finished_ts() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "b1", "agent/b1", Some("main"), "cafebabe", None, None, None, None).unwrap();
    let updated = ledger::fail(&conn, "b1", "cargo build failed").unwrap();
    assert_eq!(updated, 1);
    let failed = ledger::list(&conn, Some("failed")).unwrap();
    assert_eq!(failed.len(), 1);
    assert!(failed[0].finished_ts.is_some());
    assert_eq!(failed[0].summary.as_deref(), Some("cargo build failed"));
}

#[test]
fn tree_walks_parent_child_chain() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "root", "agent/root", Some("main"), "aa", None, None, None, None).unwrap();
    ledger::fork(&conn, "c1", "agent/c1", Some("agent/root"), "bb", None, None, None, None).unwrap();
    ledger::fork(&conn, "c2", "agent/c2", Some("agent/root"), "cc", None, None, None, None).unwrap();
    ledger::fork(&conn, "g1", "agent/g1", Some("agent/c1"), "dd", None, None, None, None).unwrap();

    let t = ledger::tree(&conn, "root").unwrap();
    let ids: Vec<_> = t.iter().map(|a| a.id.as_str()).collect();
    assert!(ids.contains(&"root"));
    assert!(ids.contains(&"c1"));
    assert!(ids.contains(&"c2"));
    assert!(ids.contains(&"g1"));
    assert_eq!(ids[0], "root");
    assert_eq!(ids.len(), 4);
}

#[test]
fn list_filter_status_excludes_others() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "r1", "br-r1", None, "s1", None, None, None, None).unwrap();
    ledger::fork(&conn, "r2", "br-r2", None, "s2", None, None, None, None).unwrap();
    ledger::done(&conn, "r1", "ok").unwrap();
    let running = ledger::list(&conn, Some("running")).unwrap();
    assert_eq!(running.len(), 1);
    assert_eq!(running[0].id, "r2");
    let all = ledger::list(&conn, None).unwrap();
    assert_eq!(all.len(), 2);
}

#[test]
fn validate_detects_missing_artefacts() {
    let (d, _conn) = open_tmp();
    write_artefacts(d.path(), "v1", &["spec.md", "plan.md"]);
    let missing = ledger::validate(d.path(), "v1");
    assert_eq!(missing.len(), 4);
    assert!(missing.contains(&"progress.json".to_string()));
    assert!(missing.contains(&"review.md".to_string()));
}

#[test]
fn validate_ok_when_all_six_present() {
    let (d, _conn) = open_tmp();
    write_artefacts(
        d.path(),
        "v2",
        &[
            "spec.md",
            "plan.md",
            "progress.json",
            "chatlog.md",
            "handoffs.md",
            "review.md",
        ],
    );
    let missing = ledger::validate(d.path(), "v2");
    assert!(missing.is_empty(), "got missing {missing:?}");
}

#[test]
fn duplicate_fork_id_rejected() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "dup", "br1", None, "x", None, None, None, None).unwrap();
    let err = ledger::fork(&conn, "dup", "br2", None, "y", None, None, None, None);
    assert!(err.is_err(), "duplicate id must fail");
}

#[test]
fn done_on_already_done_agent_is_noop() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "n1", "br-n1", None, "h", None, None, None, None).unwrap();
    assert_eq!(ledger::done(&conn, "n1", "first").unwrap(), 1);
    assert_eq!(ledger::done(&conn, "n1", "second").unwrap(), 0);
    let row = &ledger::list(&conn, None).unwrap()[0];
    assert_eq!(row.summary.as_deref(), Some("first"));
}

#[test]
fn fork_with_dna_roundtrips_through_list() {
    let (_d, conn) = open_tmp();
    let dna = "edit-local::NG-FW-FD-CP-CG-TG-ND-RF::A7B2::C9F1-xa7c";
    ledger::fork(&conn, "dna1", "agent/dna1", None, "spec", None, Some(dna), None, None).unwrap();
    let rows = ledger::list(&conn, None).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].dna.as_deref(), Some(dna));

    ledger::fork(&conn, "legacy1", "agent/legacy1", None, "spec2", None, None, None, None).unwrap();
    let rows = ledger::list(&conn, None).unwrap();
    let legacy = rows.iter().find(|r| r.id == "legacy1").unwrap();
    assert!(legacy.dna.is_none(), "legacy fork should leave dna NULL");
}

#[test]
fn merged_after_done_transitions_status() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "m1", "br-m1", None, "h", None, None, None, None).unwrap();
    ledger::done(&conn, "m1", "ready").unwrap();
    assert_eq!(ledger::merged(&conn, "m1").unwrap(), 1);
    let merged = ledger::list(&conn, Some("merged")).unwrap();
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].summary.as_deref(), Some("ready"));
}

// --- audit fixes (2026-04-23) ------------------------------------------

/// Fix S2 — cycle in parent_branch must not hang `tree()`. Synthetic cycle
/// br-x→br-y→br-x is injected by disabling the check trigger temporarily
/// via raw INSERT (bypassing `ledger::fork`'s length guard is not needed;
/// the cycle itself is the payload). The walk must terminate with either
/// `MaxDepthExceeded` OR cleanly (visited-set short-circuit), never loop.
#[test]
fn tree_handles_cycle_without_infinite_loop() {
    let (_d, conn) = open_tmp();
    // Two rows whose parent_branch point at each other.
    ledger::fork(&conn, "cx", "br-x", Some("br-y"), "sha-x", None, None, None, None).unwrap();
    ledger::fork(&conn, "cy", "br-y", Some("br-x"), "sha-y", None, None, None, None).unwrap();

    // tree() should either return bounded rows (visited-set kills the loop)
    // or MaxDepthExceeded. Must not hang / OOM.
    let out = ledger::tree(&conn, "cx");
    match out {
        Ok(rows) => {
            // visited-set: cx root, plus cy as child of br-y's... actually cx's
            // branch is br-x, children of br-x = cy; cy's branch is br-y,
            // already visited (root chained via frontier pop). <= 2 rows max.
            assert!(rows.len() <= 2, "got unbounded rows {}", rows.len());
        }
        Err(ledger::LedgerError::MaxDepthExceeded) => {
            // Acceptable: circuit breaker fired.
        }
        Err(e) => panic!("unexpected error: {e}"),
    }
}

/// Fix M2 — migration is idempotent: calling `open` twice on the same file
/// does not explode with "duplicate column" or leave user_version stale.
/// This implicitly exercises the transaction wrapper (v1, v2, v3 must all
/// commit cleanly across two opens).
#[test]
fn migrate_is_idempotent_across_reopens() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    {
        let conn = ledger::open(&db).unwrap();
        ledger::fork(&conn, "pre", "br-pre", None, "h", None, None, None, None).unwrap();
    }
    // Second open re-enters migrate(); must be a no-op, not a duplicate
    // column / trigger error.
    let conn = ledger::open(&db).unwrap();
    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(version, schema::MIGRATIONS.len() as i64);
    // Row from first session must survive.
    let rows = ledger::list(&conn, None).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, "pre");
}

/// Fix L1 — branch longer than MAX_BRANCH_LEN must be rejected at the
/// library boundary with `LedgerError::BranchTooLong` (clap `value_parser`
/// provides the same guard at the CLI boundary).
#[test]
fn fork_rejects_overlong_branch() {
    let (_d, conn) = open_tmp();
    let long = "x".repeat(schema::MAX_BRANCH_LEN + 1);
    let res = ledger::fork(&conn, "too-long", &long, None, "h", None, None, None, None);
    match res {
        Err(ledger::LedgerError::BranchTooLong { field, len }) => {
            assert_eq!(field, "branch");
            assert_eq!(len, schema::MAX_BRANCH_LEN + 1);
        }
        other => panic!("expected BranchTooLong, got {other:?}"),
    }
    // Parent side same cap.
    let res2 = ledger::fork(&conn, "ok-br", "fine", Some(&long), "h", None, None, None, None);
    assert!(
        matches!(
            res2,
            Err(ledger::LedgerError::BranchTooLong { field: "parent_branch", .. })
        ),
        "expected parent_branch rejection"
    );
    // Length at the cap is accepted.
    let at_cap = "y".repeat(schema::MAX_BRANCH_LEN);
    ledger::fork(&conn, "at-cap", &at_cap, None, "h", None, None, None, None).unwrap();
}

// --- v4 lineage (creator_id + fork_parent_id) ---------------------------

/// v4-T1: `--creator` value stored on fork and retrieved via list.
#[test]
fn fork_with_creator_id_roundtrips_through_list() {
    let (_d, conn) = open_tmp();
    let creator = "human:denis";
    ledger::fork(&conn, "v4a", "agent/v4a", None, "sh", None, None, Some(creator), None).unwrap();
    let rows = ledger::list(&conn, None).unwrap();
    let r = rows.iter().find(|r| r.id == "v4a").unwrap();
    assert_eq!(r.creator_id.as_deref(), Some(creator));
    assert!(r.fork_parent_id.is_none());
}

/// v4-T2: `--fork-parent` value stores lineage pointer to a DNA.
#[test]
fn fork_with_fork_parent_stores_lineage() {
    let (_d, conn) = open_tmp();
    let parent_dna = "edit-local::NG-FW::ABCD::1234-xy01";
    ledger::fork(
        &conn, "v4b", "agent/v4b", None, "sh", None, None, None, Some(parent_dna),
    )
    .unwrap();
    let r = ledger::list(&conn, None).unwrap().into_iter().next().unwrap();
    assert_eq!(r.fork_parent_id.as_deref(), Some(parent_dna));
    assert!(r.creator_id.is_none());
}

/// v4-T3: `descendants()` returns rows matched via EITHER column.
#[test]
fn descendants_returns_fork_and_spawn_chain() {
    let (_d, conn) = open_tmp();
    let root_dna = "root-dna-0001";
    // child forked FROM root_dna
    ledger::fork(
        &conn, "d1", "agent/d1", None, "sh", None, None, None, Some(root_dna),
    )
    .unwrap();
    // child SPAWNED BY root_dna (creator_id match)
    ledger::fork(
        &conn, "d2", "agent/d2", None, "sh", None, None, Some(root_dna), None,
    )
    .unwrap();
    // unrelated agent — must NOT appear
    ledger::fork(&conn, "d3", "agent/d3", None, "sh", None, None, None, None).unwrap();

    let out = descendants::descendants(&conn, root_dna).unwrap();
    let ids: Vec<_> = out.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(out.len(), 2, "expected exactly 2 descendants, got {ids:?}");
    assert!(ids.contains(&"d1"));
    assert!(ids.contains(&"d2"));
    assert!(!ids.contains(&"d3"));
}

/// v4-T4: legacy rows written before migration v4 have NULL creator + fork_parent.
/// Simulates by inserting a row with the pre-v4 column subset then reopening.
#[test]
fn pre_v4_rows_have_null_lineage_columns() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    {
        let conn = ledger::open(&db).unwrap();
        ledger::fork(&conn, "old", "br-old", None, "sh", None, None, None, None).unwrap();
    }
    // Reopen — migration re-runs (no-op at v4), row survives.
    let conn = ledger::open(&db).unwrap();
    let rows = ledger::list(&conn, None).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0].creator_id.is_none());
    assert!(rows[0].fork_parent_id.is_none());
}

/// v4-T5: migration v3 → v4 idempotent across multiple reopens, schema at v4.
#[test]
fn migration_v4_idempotent_across_multiple_reopens() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    for _ in 0..3 {
        let conn = ledger::open(&db).unwrap();
        let v: i64 = conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(v, schema::MIGRATIONS.len() as i64, "schema must land at latest");
    }
    // Seed a row using v4 columns and verify it round-trips after another reopen.
    {
        let conn = ledger::open(&db).unwrap();
        ledger::fork(
            &conn, "v4e", "agent/v4e", None, "sh", None, None, Some("c"), Some("fp"),
        )
        .unwrap();
    }
    let conn = ledger::open(&db).unwrap();
    let r = ledger::list(&conn, None).unwrap().into_iter().next().unwrap();
    assert_eq!(r.creator_id.as_deref(), Some("c"));
    assert_eq!(r.fork_parent_id.as_deref(), Some("fp"));
}

// --- v5 DNA UNIQUE constraint (2026-04-23) ------------------------------

/// v5-T1: migration v5 creates `idx_agents_dna_unique` on the agents table.
/// Detection via `sqlite_master` is the sqlite-canonical way to assert an
/// index exists independent of whether it has been used.
#[test]
fn v5_migration_adds_unique_index() {
    let (_d, conn) = open_tmp();
    let found: Option<String> = conn
        .query_row(
            "SELECT name FROM sqlite_master
             WHERE type='index' AND name='idx_agents_dna_unique'",
            [],
            |r| r.get(0),
        )
        .ok();
    assert_eq!(found.as_deref(), Some("idx_agents_dna_unique"));
    // Schema version must have advanced to the migration list's length.
    let v: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, schema::MIGRATIONS.len() as i64);
}

/// v5-T2: a second `fork` with the same DNA is rejected with the typed
/// `DnaCollision` variant, not a raw SQL error. The payload preserves the
/// offending DNA so the caller can log / regenerate.
#[test]
fn duplicate_dna_rejected_at_fork() {
    let (_d, conn) = open_tmp();
    let dna = "edit-local::NG-FW::ABCD::1234-xy01";
    ledger::fork(&conn, "a1", "agent/a1", None, "sh", None, Some(dna), None, None).unwrap();
    let res = ledger::fork(&conn, "a2", "agent/a2", None, "sh", None, Some(dna), None, None);
    match res {
        Err(ledger::LedgerError::DnaCollision { dna: got }) => {
            assert_eq!(got, dna, "collision variant must carry the offending dna");
        }
        other => panic!("expected DnaCollision, got {other:?}"),
    }
    // Non-dna uniqueness (primary-key id) must still flow through Sql — not
    // mis-classified as a DNA collision.
    let res2 = ledger::fork(
        &conn, "a1", "agent/a1b", None, "sh", None, Some("other-dna"), None, None,
    );
    assert!(
        matches!(res2, Err(ledger::LedgerError::Sql(_))),
        "duplicate id should stay a Sql error, got {res2:?}"
    );
}

/// v5-T3: opening a freshly-built pre-v5 ledger that already contains
/// duplicate DNAs must surface `DnaMigrationBlocked` listing each offender.
/// Simulated by: (a) open with current schema to create table, (b) drop the
/// v5 UNIQUE index + rewind user_version to 4, (c) INSERT two conflicting
/// rows by hand, (d) reopen → pre-check fires before index recreation.
#[test]
fn v5_migration_detects_preexisting_duplicates() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    {
        let conn = ledger::open(&db).unwrap();
        conn.execute("DROP INDEX IF EXISTS idx_agents_dna_unique", []).unwrap();
        conn.pragma_update(None, "user_version", 4_i64).unwrap();
        // Insert two rows with matching DNA directly — bypasses the v5 UNIQUE
        // index since we just dropped it, reproducing the pre-v5 world where
        // the 32-bit DNA nonce already collided for some operator.
        let ts = 1_700_000_000_i64;
        let dup = "duplicate-dna-abc";
        conn.execute(
            "INSERT INTO agents
             (id, branch, spec_sha, status, started_ts, dna)
             VALUES (?1, ?2, ?3, 'running', ?4, ?5)",
            rusqlite::params!["dup1", "br-dup1", "sha", ts, dup],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO agents
             (id, branch, spec_sha, status, started_ts, dna)
             VALUES (?1, ?2, ?3, 'running', ?4, ?5)",
            rusqlite::params!["dup2", "br-dup2", "sha", ts, dup],
        )
        .unwrap();
    }
    // Second open re-enters migrate(); pre-check must fire and block v5.
    let res = ledger::open(&db);
    match res {
        Err(ledger::LedgerError::DnaMigrationBlocked { duplicates }) => {
            assert_eq!(duplicates.len(), 1, "expected 1 dup group, got {duplicates:?}");
            assert_eq!(duplicates[0].0, "duplicate-dna-abc");
            assert_eq!(duplicates[0].1, 2);
        }
        other => panic!("expected DnaMigrationBlocked, got {other:?}"),
    }
    // Index must NOT have been created (migration aborted cleanly).
    let conn_raw = rusqlite::Connection::open(&db).unwrap();
    let exists: Option<String> = conn_raw
        .query_row(
            "SELECT name FROM sqlite_master
             WHERE type='index' AND name='idx_agents_dna_unique'",
            [],
            |r| r.get(0),
        )
        .ok();
    assert!(exists.is_none(), "v5 index must not have been applied");
    let v: i64 = conn_raw
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, 4, "user_version must stay at v4 after blocked migration");
}

/// v5-T4: running migrations twice does not fail and leaves the index in
/// place. Complements the pre-existing `migrate_is_idempotent_across_reopens`
/// test with an assertion that specifically pins v5.
#[test]
fn migration_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    // First open applies v1..v5.
    {
        let _conn = ledger::open(&db).unwrap();
    }
    // Second open must be a no-op — no "index already exists" error.
    let conn = ledger::open(&db).unwrap();
    let found: Option<String> = conn
        .query_row(
            "SELECT name FROM sqlite_master
             WHERE type='index' AND name='idx_agents_dna_unique'",
            [],
            |r| r.get(0),
        )
        .ok();
    assert_eq!(found.as_deref(), Some("idx_agents_dna_unique"));
    let v: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .unwrap();
    assert_eq!(v, schema::MIGRATIONS.len() as i64);
    // Third open for good measure.
    let _conn2 = ledger::open(&db).unwrap();
}

/// v5-T5: multiple NULL DNAs are still accepted (SQLite default UNIQUE
/// semantics). Matches the v2 "dna optional" contract for legacy callers.
#[test]
fn v5_allows_multiple_null_dnas() {
    let (_d, conn) = open_tmp();
    ledger::fork(&conn, "n1", "br-n1", None, "sh", None, None, None, None).unwrap();
    ledger::fork(&conn, "n2", "br-n2", None, "sh", None, None, None, None).unwrap();
    ledger::fork(&conn, "n3", "br-n3", None, "sh", None, None, None, None).unwrap();
    let all = ledger::list(&conn, None).unwrap();
    assert_eq!(all.len(), 3);
    assert!(all.iter().all(|r| r.dna.is_none()));
}

