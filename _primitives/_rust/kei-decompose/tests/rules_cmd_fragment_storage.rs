//! Integration test: decompose-rules writes real fragment files and registers
//! real paths in the SQLite registry. Validates Path 1 fix for Wave 14d.
//!
//! Verify criterion: after `rules_cmd::run(...)`, every active `rule` row in
//! the registry has a `path` that exists on disk and whose content matches
//! the registered fragment body.

use std::path::PathBuf;
use tempfile::TempDir;

use kei_decompose::rules_cmd;

// ── helper: minimal rule markdown file ───────────────────────────────────────

fn write_rule_md(dir: &std::path::Path, stem: &str, content: &str) -> PathBuf {
    let path = dir.join(format!("{stem}.md"));
    std::fs::write(&path, content).unwrap();
    path
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[test]
fn fragment_files_are_written_to_disk() {
    let tmp = TempDir::new().unwrap();
    let rules_dir = tmp.path().join("rules");
    let frags_dir = tmp.path().join("fragments");
    let db_path = tmp.path().join("registry.sqlite");
    std::fs::create_dir_all(&rules_dir).unwrap();

    write_rule_md(
        &rules_dir,
        "karpathy-behavioral",
        "# Karpathy\n\n## Think Before Coding\n\nBe explicit.\n\n## Surgical Changes\n\nOnly change what's needed.\n",
    );

    let result = rules_cmd::run(
        Some(rules_dir.clone()),
        Some(db_path.clone()),
        Some(frags_dir.clone()),
        false,
        false,
    );
    // ExitCode::SUCCESS == ExitCode::from(0); compare via Debug string.
    assert_eq!(format!("{result:?}"), "ExitCode(unix_exit_status(0))", "run must succeed");

    // Verify fragment files exist on disk.
    let think_file = frags_dir.join("karpathy-behavioral__think-before-coding.md");
    let surgical_file = frags_dir.join("karpathy-behavioral__surgical-changes.md");
    assert!(think_file.exists(), "think-before-coding fragment file must exist");
    assert!(surgical_file.exists(), "surgical-changes fragment file must exist");

    // Verify file contents match what was parsed.
    let think_body = std::fs::read_to_string(&think_file).unwrap();
    assert!(think_body.contains("Be explicit."), "fragment body must contain section text");
}

#[test]
fn registry_path_column_is_real_filesystem_path() {
    let tmp = TempDir::new().unwrap();
    let rules_dir = tmp.path().join("rules");
    let frags_dir = tmp.path().join("fragments");
    let db_path = tmp.path().join("registry.sqlite");
    std::fs::create_dir_all(&rules_dir).unwrap();

    write_rule_md(
        &rules_dir,
        "code-style",
        "# Code Style\n\n## Constructor Pattern\n\n1 file = 1 class.\n",
    );

    rules_cmd::run(
        Some(rules_dir),
        Some(db_path.clone()),
        Some(frags_dir.clone()),
        false,
        false,
    );

    // Query registry directly.
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    let path: String = conn
        .query_row(
            "SELECT path FROM blocks WHERE block_type='rule' AND superseded_by IS NULL LIMIT 1",
            [],
            |r| r.get(0),
        )
        .expect("at least one rule row must exist");

    // The path must NOT contain '::' — that was the old broken format.
    assert!(
        !path.contains("::"),
        "path must not be a logical key; got: {path}"
    );

    // The path must exist on disk.
    assert!(
        std::path::Path::new(&path).exists(),
        "registry path must be a real file; got: {path}"
    );
}

#[test]
fn assembler_registry_client_reads_fragment_via_real_path() {
    let tmp = TempDir::new().unwrap();
    let rules_dir = tmp.path().join("rules");
    let frags_dir = tmp.path().join("fragments");
    let db_path = tmp.path().join("registry.sqlite");
    std::fs::create_dir_all(&rules_dir).unwrap();

    write_rule_md(
        &rules_dir,
        "no-downgrade-constructive",
        "# No Downgrade\n\n## The Rule\n\nAlways return solutions.\n",
    );

    rules_cmd::run(
        Some(rules_dir),
        Some(db_path.clone()),
        Some(frags_dir.clone()),
        false,
        false,
    );

    // Simulate what assembler/registry_client::find_rule does:
    // open DB, query path, read path.
    let conn = rusqlite::Connection::open_with_flags(
        &db_path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .unwrap();
    let path: String = conn
        .query_row(
            "SELECT path FROM blocks WHERE name='no-downgrade-constructive::the-rule' LIMIT 1",
            [],
            |r| r.get(0),
        )
        .expect("row must exist");
    let body = std::fs::read_to_string(&path).expect("fragment file must be readable");
    assert!(body.contains("Always return solutions."), "body must contain fragment text");
}

#[test]
fn dry_run_does_not_write_files_or_registry() {
    let tmp = TempDir::new().unwrap();
    let rules_dir = tmp.path().join("rules");
    let frags_dir = tmp.path().join("fragments");
    let db_path = tmp.path().join("registry.sqlite");
    std::fs::create_dir_all(&rules_dir).unwrap();

    write_rule_md(
        &rules_dir,
        "dry-test-rule",
        "# Dry Test\n\n## Section One\n\nBody here.\n",
    );

    rules_cmd::run(
        Some(rules_dir),
        Some(db_path.clone()),
        Some(frags_dir.clone()),
        true, // dry_run = true
        false,
    );

    // No fragment files should exist.
    assert!(!frags_dir.exists() || std::fs::read_dir(&frags_dir).unwrap().count() == 0,
        "dry_run must not write fragment files");

    // No registry should exist.
    assert!(!db_path.exists(), "dry_run must not create registry");
}

#[test]
fn idempotent_rerun_unchanged_fragment_is_not_rewritten() {
    let tmp = TempDir::new().unwrap();
    let rules_dir = tmp.path().join("rules");
    let frags_dir = tmp.path().join("fragments");
    let db_path = tmp.path().join("registry.sqlite");
    std::fs::create_dir_all(&rules_dir).unwrap();

    write_rule_md(
        &rules_dir,
        "idempotency-rule",
        "# Idem\n\n## Section\n\nStable content.\n",
    );

    rules_cmd::run(
        Some(rules_dir.clone()),
        Some(db_path.clone()),
        Some(frags_dir.clone()),
        false,
        false,
    );

    let frag_path = frags_dir.join("idempotency-rule__section.md");
    let mtime1 = std::fs::metadata(&frag_path).unwrap().modified().unwrap();

    // Small sleep to ensure mtime would differ if file were rewritten.
    std::thread::sleep(std::time::Duration::from_millis(50));

    rules_cmd::run(
        Some(rules_dir),
        Some(db_path),
        Some(frags_dir),
        false,
        false,
    );

    let mtime2 = std::fs::metadata(&frag_path).unwrap().modified().unwrap();
    assert_eq!(mtime1, mtime2, "unchanged fragment must not be rewritten on second run");
}
