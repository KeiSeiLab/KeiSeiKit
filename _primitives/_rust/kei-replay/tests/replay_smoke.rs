//! Smoke tests for kei-replay.
//!
//! Covers: happy-path replay, missing DNA, drift detection, diff.
//!
//! Each test builds its own isolated tempdir with:
//!   - SQLite ledger seeded with the relevant `agents` row
//!   - `<worktree>/tasks/<agent-id>/task.toml`
//!   - `<kit_root>/_roles/<role>.toml`
//!   - `<kit_root>/_capabilities/<cat>/<slug>/text.md`
//! Then calls `replay::replay` / `diff::diff` directly (skips the CLI layer).

use kei_agent_runtime::capability::TaskSpec;
use kei_agent_runtime::dna::Dna;
use kei_agent_runtime::role::ResolvedRole;
use kei_replay::{diff, replay};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

struct Fixture {
    _tmp: TempDir,
    db: PathBuf,
    kit_root: PathBuf,
    agent_id: String,
    dna_str: String,
    body: String,
}

fn resolved_fake() -> ResolvedRole {
    ResolvedRole {
        required: vec!["policy::no-git-ops".into(), "output::report-format".into()],
        warnings: Vec::new(),
    }
}

fn write_kit(kit: &Path) {
    std::fs::create_dir_all(kit.join("_capabilities/policy/no-git-ops")).unwrap();
    std::fs::write(
        kit.join("_capabilities/policy/no-git-ops/text.md"),
        "## No git\n\nYou must not git.\n",
    )
    .unwrap();
    std::fs::create_dir_all(kit.join("_capabilities/output/report-format")).unwrap();
    std::fs::write(
        kit.join("_capabilities/output/report-format/text.md"),
        "## Report\n\nEmit a report.\n",
    )
    .unwrap();
    std::fs::create_dir_all(kit.join("_roles")).unwrap();
    std::fs::write(
        kit.join("_roles/fake.toml"),
        r#"
[role]
name = "fake"

[capabilities]
required = ["policy::no-git-ops", "output::report-format"]
"#,
    )
    .unwrap();
}

fn seed_ledger(db: &Path, agent_id: &str, dna: &str, worktree: &str) {
    let conn = Connection::open(db).unwrap();
    // Minimal schema for what kei-replay reads. Mirrors kei-ledger v4 cols.
    conn.execute_batch(
        "CREATE TABLE agents (
            id TEXT PRIMARY KEY,
            branch TEXT NOT NULL,
            parent_branch TEXT,
            spec_sha TEXT NOT NULL,
            status TEXT NOT NULL,
            started_ts INTEGER NOT NULL,
            finished_ts INTEGER,
            summary TEXT,
            worktree_path TEXT,
            dna TEXT,
            creator_id TEXT,
            fork_parent_id TEXT
        );",
    )
    .unwrap();
    conn.execute(
        "INSERT INTO agents (id, branch, spec_sha, status, started_ts, worktree_path, dna)
         VALUES (?1, 'agent/test', 'abc', 'running', 0, ?2, ?3)",
        params![agent_id, worktree, dna],
    )
    .unwrap();
}

fn write_task_toml(worktree: &Path, agent_id: &str, body: &str) -> PathBuf {
    let dir = worktree.join("tasks").join(agent_id);
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("task.toml");
    // Explicit TOML literal: body.text uses triple-quoted block to survive
    // any special chars in body strings across tests.
    let toml = format!(
        r#"[task]
role = "fake"
agent-id = "{agent_id}"

[body]
text = """{body}"""
"#
    );
    std::fs::write(&p, toml).unwrap();
    p
}

fn build_dna(task: &TaskSpec) -> String {
    Dna::compose(task, &resolved_fake()).render()
}

fn make_fixture(body: &str) -> Fixture {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().to_path_buf();
    let kit_root = root.join("kit");
    let worktree = root.join("worktree");
    std::fs::create_dir_all(&worktree).unwrap();
    write_kit(&kit_root);

    let agent_id = "test-agent-01".to_string();
    let mut task = TaskSpec::default();
    task.task.role = "fake".into();
    task.task.agent_id = agent_id.clone();
    task.body.text = body.to_string();

    write_task_toml(&worktree, &agent_id, body);
    let dna_str = build_dna(&task);

    let db = root.join("ledger.sqlite");
    seed_ledger(&db, &agent_id, &dna_str, worktree.to_str().unwrap());

    Fixture { _tmp: tmp, db, kit_root, agent_id, dna_str, body: body.to_string() }
}

#[test]
fn replay_happy_path_reconstructs_prompt_and_matches_body_hash() {
    let f = make_fixture("Port the kei-forge templating subsystem.");
    let r = replay::replay(&f.db, &f.dna_str, None, &f.kit_root).expect("replay");
    assert!(r.body_hash_matches, "body hash must match at the happy path");
    assert!(r.composed_prompt.contains("You must not git"));
    assert!(r.composed_prompt.contains("Emit a report"));
    assert!(r.composed_prompt.contains(&f.body));
    assert_eq!(r.dna.role, "fake");
    // task.toml text is echoed back verbatim
    assert!(r.task_toml_text.contains(&f.agent_id));
}

#[test]
fn replay_unknown_dna_errors_with_not_found_message() {
    let f = make_fixture("body");
    // Use well-formed but unseeded DNA — parse passes, lookup must fail.
    let bogus = "fake::NG-RF::DEADBEEF::BAADF00D-12345678";
    let err = replay::replay(&f.db, bogus, None, &f.kit_root).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("not found"), "expected 'not found', got: {msg}");
}

#[test]
fn replay_detects_body_drift_when_task_toml_mutated_after_spawn() {
    let f = make_fixture("original body");
    // Mutate the on-disk task.toml to simulate schema drift since spawn.
    let task_path = PathBuf::from(&f.db)
        .parent()
        .unwrap()
        .join("worktree/tasks")
        .join(&f.agent_id)
        .join("task.toml");
    let mutated = format!(
        r#"[task]
role = "fake"
agent-id = "{}"

[body]
text = """MUTATED body — drift injected"""
"#,
        f.agent_id
    );
    std::fs::write(&task_path, mutated).unwrap();

    let r = replay::replay(&f.db, &f.dna_str, None, &f.kit_root).expect("replay");
    assert!(
        !r.body_hash_matches,
        "drift must be detected: dna={} recomputed={}",
        r.dna.body_hash, r.recomputed_body_hash
    );
    assert_ne!(r.dna.body_hash, r.recomputed_body_hash);
}

#[test]
fn diff_two_dnas_flags_every_changed_facet() {
    let f1 = make_fixture("body-one");
    let f2 = make_fixture("body-two");
    let d = diff::diff(&f1.dna_str, &f2.dna_str).expect("diff");
    // Same role + same caps + same (empty) scope + different body => different body_hash
    assert!(!d.role_changed);
    assert!(!d.caps_changed);
    assert!(!d.scope_changed);
    assert!(d.body_changed, "body must differ between the two fixtures");
    // Different invocations => nonces almost always differ.
    assert!(d.nonce_changed || !d.is_identical());
    assert!(!d.is_same_composition(), "body differs => composition differs");
    let rendered = d.render();
    assert!(rendered.contains("CHANGED"));
}

#[test]
fn diff_identical_dna_strings_report_as_identical() {
    let f = make_fixture("same");
    let d = diff::diff(&f.dna_str, &f.dna_str).expect("diff");
    assert!(d.is_identical());
    assert!(d.is_same_composition());
    assert!(!d.body_changed);
    assert!(!d.nonce_changed);
}

#[test]
fn replay_honours_explicit_task_override_bypassing_ledger_worktree() {
    // Fixture's task.toml lives at worktree/tasks/<id>/task.toml; copy it to
    // a side path and pass --task override. Ledger row is still required for
    // the DNA lookup (to assert it was spawned), but file path is overridden.
    let f = make_fixture("override test");
    let side = f.kit_root.parent().unwrap().join("side-task.toml");
    let orig = f.kit_root.parent().unwrap().join("worktree/tasks").join(&f.agent_id).join("task.toml");
    std::fs::copy(&orig, &side).unwrap();
    let r = replay::replay(&f.db, &f.dna_str, Some(&side), &f.kit_root).expect("replay");
    assert!(r.body_hash_matches);
}
