//! v7 CLI binary tests for `record-cost` (Wave 44c, 2026-04-24).
//!
//! Constructor Pattern: extracted from `v6_cost.rs` so each test file
//! stays under the 200-LOC ceiling. Loads source modules via `#[path]`
//! to avoid forcing all callers through the public lib API.

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
#[path = "../src/cost.rs"]
mod cost;

use std::process::Command;

/// v6-T6: `record-cost` CLI subcommand round-trips a real binary build.
/// Under additive semantics, a single call on a fresh row still yields
/// the call's `cents` value (0 + N = N).
#[test]
fn record_cost_cli_roundtrips_through_binary() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    {
        let conn = ledger::open(&db).unwrap();
        ledger::fork(&conn, "cli-1", "br-cli-1", None, "sha", None, None, None, None).unwrap();
    }
    let bin = env!("CARGO_BIN_EXE_kei-ledger");
    let out = Command::new(bin)
        .args([
            "--db", db.to_str().unwrap(),
            "record-cost", "cli-1",
            "--cents", "777",
            "--provider", "anthropic",
            "--model", "claude-haiku-4-5-20251001",
        ])
        .output()
        .expect("kei-ledger binary failed to spawn");
    assert!(
        out.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"ok\":true"), "stdout={stdout}");
    assert!(stdout.contains("\"agent_id\":\"cli-1\""), "stdout={stdout}");
    assert!(stdout.contains("\"total_cost_cents\":777"), "stdout={stdout}");
    let conn = ledger::open(&db).unwrap();
    let (c, _, _) = cost::read_cost(&conn, "cli-1").unwrap().unwrap();
    assert_eq!(c, 777);
}

/// v7-T6b (Wave 44c): `record-cost` CLI defaults to ADDITIVE; three
/// successive calls accumulate.
#[test]
fn record_cost_cli_accumulates_by_default() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    {
        let conn = ledger::open(&db).unwrap();
        ledger::fork(&conn, "cli-acc", "br-cli-acc", None, "sha", None, None, None, None)
            .unwrap();
    }
    let bin = env!("CARGO_BIN_EXE_kei-ledger");
    for cents in ["100", "250", "75"] {
        let out = Command::new(bin)
            .args([
                "--db", db.to_str().unwrap(),
                "record-cost", "cli-acc",
                "--cents", cents,
                "--provider", "anthropic",
                "--model", "claude-haiku-4-5-20251001",
            ])
            .output()
            .expect("kei-ledger binary failed to spawn");
        assert!(out.status.success());
    }
    let conn = ledger::open(&db).unwrap();
    let (c, _, _) = cost::read_cost(&conn, "cli-acc").unwrap().unwrap();
    assert_eq!(c, 425, "100 + 250 + 75 accumulates");
}

/// v7-T6c: `--replace` flag restores last-write-wins behavior for
/// callers that need it (retry / amend flows).
#[test]
fn record_cost_cli_replace_flag_overrides() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    {
        let conn = ledger::open(&db).unwrap();
        ledger::fork(&conn, "cli-rep", "br-cli-rep", None, "sha", None, None, None, None)
            .unwrap();
        cost::record_cost(&conn, "cli-rep", 999, "anthropic", "claude").unwrap();
    }
    let bin = env!("CARGO_BIN_EXE_kei-ledger");
    let out = Command::new(bin)
        .args([
            "--db", db.to_str().unwrap(),
            "record-cost", "cli-rep",
            "--cents", "50",
            "--provider", "openai",
            "--model", "gpt-4o",
            "--replace",
        ])
        .output()
        .expect("kei-ledger binary failed to spawn");
    assert!(out.status.success());
    let conn = ledger::open(&db).unwrap();
    let (c, p, _) = cost::read_cost(&conn, "cli-rep").unwrap().unwrap();
    assert_eq!(c, 50, "--replace overrides 999, not adds to it");
    assert_eq!(p, "openai");
}

/// v6-T7: `record-cost` CLI on a missing agent prints to stderr and exits 1.
#[test]
fn record_cost_cli_missing_agent_exits_nonzero() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("ledger.sqlite");
    let _ = ledger::open(&db).unwrap();
    let bin = env!("CARGO_BIN_EXE_kei-ledger");
    let out = Command::new(bin)
        .args([
            "--db", db.to_str().unwrap(),
            "record-cost", "ghost",
            "--cents", "10",
            "--provider", "x",
            "--model", "y",
        ])
        .output()
        .expect("kei-ledger binary failed to spawn");
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("no agent with id ghost"), "stderr={stderr}");
}
