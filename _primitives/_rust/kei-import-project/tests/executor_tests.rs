//! Integration tests for executor.
//! All fixtures use synthetic TempDir-built ledger DBs.

use kei_import_project::executor::{build_executor_plan, prereg_phases, ExecutionStatus};
use kei_import_project::plan_parser::{ParsedModule, ParsedPhase, ParsedPlan};
use tempfile::TempDir;

fn make_plan(phases: &[(&str, &str, u8)]) -> ParsedPlan {
    ParsedPlan {
        project_name: "test-project".to_owned(),
        source_repo: "file:///tmp/test-repo".to_owned(),
        phases: phases
            .iter()
            .map(|(id, family, pri)| ParsedPhase {
                id: id.to_string(),
                trait_family: family.to_string(),
                priority: *pri,
                status: "scaffolding".to_owned(),
                modules: vec![ParsedModule {
                    name: format!("mod-{}", id.to_lowercase().replace('.', "-")),
                    confidence: 0.80,
                }],
            })
            .collect(),
        unmatched: vec![],
    }
}

#[test]
fn executor_plan_one_record_per_phase() {
    let plan = make_plan(&[("P0.1", "MemoryBackend", 0), ("P1.1", "ComputeProvider", 1)]);
    let ep = build_executor_plan(&plan, None).unwrap();
    assert_eq!(ep.records.len(), 2);
    assert_eq!(ep.prompts.len(), 2);
    assert_eq!(ep.records[0].phase_id, "P0.1");
    assert_eq!(ep.records[1].phase_id, "P1.1");
}

#[test]
fn all_records_start_as_queued() {
    let plan = make_plan(&[("P0.1", "MemoryBackend", 0)]);
    let ep = build_executor_plan(&plan, None).unwrap();
    assert!(ep.records.iter().all(|r| r.status == ExecutionStatus::Queued));
}

#[test]
fn prereg_inserts_rows_into_ledger() {
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("ledger.sqlite");

    let plan = make_plan(&[("P0.1", "MemoryBackend", 0), ("P1.1", "ComputeProvider", 1)]);
    let ep = build_executor_plan(&plan, Some(&db)).unwrap();
    prereg_phases(&ep, &db).unwrap();

    let conn = rusqlite::Connection::open(&db).unwrap();
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM agents", [], |r| r.get(0)).unwrap();
    assert_eq!(count, 2, "expected 2 ledger rows");
}

#[test]
fn prereg_is_idempotent() {
    let dir = TempDir::new().unwrap();
    let db = dir.path().join("ledger.sqlite");

    let plan = make_plan(&[("P0.1", "MemoryBackend", 0)]);
    let ep = build_executor_plan(&plan, Some(&db)).unwrap();
    prereg_phases(&ep, &db).unwrap();
    prereg_phases(&ep, &db).unwrap(); // second call must not fail or insert duplicate

    let conn = rusqlite::Connection::open(&db).unwrap();
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM agents", [], |r| r.get(0)).unwrap();
    assert_eq!(count, 1, "idempotent: still 1 row after 2 calls");
}

#[test]
fn empty_plan_produces_empty_executor_plan() {
    let plan = ParsedPlan {
        project_name: "empty".to_owned(),
        source_repo: String::new(),
        phases: vec![],
        unmatched: vec![],
    };
    let ep = build_executor_plan(&plan, None).unwrap();
    assert!(ep.records.is_empty());
    assert!(ep.prompts.is_empty());
}
