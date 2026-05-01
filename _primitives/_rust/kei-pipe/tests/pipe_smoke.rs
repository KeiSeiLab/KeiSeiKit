//! Smoke tests for the `kei-pipe` DAG runtime.
//!
//! Four scenarios:
//! 1. Happy path — 2 steps with `$step.result.id` substitution.
//! 2. Cycle detection — errors clearly.
//! 3. Unknown dependency — errors clearly.
//! 4. Resolver walks `$step.nested.sub.field` into deep paths.
//!
//! Mock atom: a shell script `mock-atom` that echoes stdin as `result`.
//! The crate name is `mockcrate`, so atom ids look like `mockcrate::echo`.

use kei_pipe::dag::{parse_dag, topo_sort, DagError};
use kei_pipe::resolve::resolve_input;
use kei_pipe::{run_dag, PipeError};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// Serialize every test that reads/writes `KEI_RUNTIME_BIN_DIR`. Without
/// this the cache/non-cache tests race and pick up each other's mock dir.
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// Create a temp dir holding a POSIX shell script `mock-atom` that, for
/// any `run-atom <verb>` invocation, echoes back `{"input": <stdin>}`.
fn mock_bin_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let bin = dir.path().join("mockcrate");
    // Read stdin verbatim and wrap in {"input": ...}. Python3 is cross-
    // platform enough for CI; we avoid jq/node to keep the dep surface
    // minimal. Smoke tests skip when python3 is missing.
    let script = r#"#!/bin/sh
exec python3 -c 'import sys, json; d = sys.stdin.read(); print(json.dumps({"input": json.loads(d)}))'
"#;
    fs::write(&bin, script).unwrap();
    let mut perms = fs::metadata(&bin).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin, perms).unwrap();
    dir
}

fn python3_available() -> bool {
    std::process::Command::new("python3")
        .arg("-c")
        .arg("print(1)")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn write_toml(dir: &std::path::Path, name: &str, body: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, body).unwrap();
    path
}

#[test]
fn happy_path_runs_two_steps_with_substitution() {
    if !python3_available() {
        eprintln!("skipping: python3 not available");
        return;
    }
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());
    let bin = mock_bin_dir();
    let work = tempfile::tempdir().unwrap();
    let dag = r#"
[[steps]]
id = "first"
atom = "mockcrate::echo"
input = { title = "Fix bug" }

[[steps]]
id = "second"
atom = "mockcrate::echo"
depends-on = ["first"]
input = { prior = "$first.result.input.title", literal = 42 }
"#;
    let path = write_toml(work.path(), "dag.toml", dag);
    std::env::set_var("KEI_RUNTIME_BIN_DIR", bin.path());
    let report = run_dag(&path).expect("run");
    std::env::remove_var("KEI_RUNTIME_BIN_DIR");
    assert!(report.final_ok(), "all steps ok; got {:?}", report.steps);
    assert_eq!(report.steps.len(), 2);
    assert_eq!(report.steps[0].id, "first");
    assert_eq!(report.steps[1].id, "second");
    let second_result = report.steps[1]
        .result
        .as_ref()
        .expect("second.result");
    // The mock echoes the RESOLVED input, so `prior` should equal "Fix bug"
    // after substitution from first.result.input.title.
    assert_eq!(
        second_result["input"]["prior"],
        Value::String("Fix bug".to_string())
    );
    assert_eq!(second_result["input"]["literal"], json!(42));
}

#[test]
fn cycle_is_detected_with_step_ids() {
    let dag = r#"
[[steps]]
id = "a"
atom = "mockcrate::echo"
depends-on = ["b"]

[[steps]]
id = "b"
atom = "mockcrate::echo"
depends-on = ["a"]
"#;
    let spec = parse_dag(dag).expect("parse");
    let err = topo_sort(&spec).expect_err("cycle");
    match err {
        DagError::Cycle(ids) => {
            assert!(ids.contains('a') && ids.contains('b'), "ids: {ids}");
        }
        other => panic!("expected Cycle, got {other:?}"),
    }
}

#[test]
fn unknown_dependency_errors_with_step_name() {
    let dag = r#"
[[steps]]
id = "only"
atom = "mockcrate::echo"
depends-on = ["ghost"]
"#;
    let spec = parse_dag(dag).expect("parse");
    let err = topo_sort(&spec).expect_err("unknown dep");
    match err {
        DagError::UnknownDep(step, dep) => {
            assert_eq!(step, "only");
            assert_eq!(dep, "ghost");
        }
        other => panic!("expected UnknownDep, got {other:?}"),
    }
}

#[test]
fn resolver_walks_nested_paths_and_array_indices() {
    // Simulate an envelope produced by a prior step.
    let mut prev: HashMap<String, Value> = HashMap::new();
    prev.insert(
        "first".into(),
        json!({
            "atom": "mockcrate::echo",
            "result": {
                "id": 17,
                "nested": { "sub": { "field": "deep" } },
                "items": ["alpha", "beta", { "tag": "third" }]
            }
        }),
    );
    let input = json!({
        "direct_id": "$first.result.id",
        "deep": "$first.result.nested.sub.field",
        "array_index": "$first.result.items.1",
        "array_object": "$first.result.items.2.tag",
        "root": "$first",
        "untouched": "plain string"
    });
    let out = resolve_input(&input, &prev).expect("resolve");
    assert_eq!(out["direct_id"], json!(17));
    assert_eq!(out["deep"], json!("deep"));
    assert_eq!(out["array_index"], json!("beta"));
    assert_eq!(out["array_object"], json!("third"));
    assert_eq!(out["root"]["atom"], json!("mockcrate::echo"));
    assert_eq!(out["untouched"], json!("plain string"));
}

#[test]
fn run_dag_rejects_unreadable_file() {
    let err = run_dag(std::path::Path::new("/no/such/file-a8f3.toml"))
        .expect_err("io error");
    assert!(matches!(err, PipeError::Read(_, _)), "got {err:?}");
}

/// Counting mock: the mock script increments a counter file every run so
/// the test can prove the cache actually bypassed the subprocess.
fn counting_mock_bin_dir(counter_path: &std::path::Path) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let bin = dir.path().join("mockcrate");
    let script = format!(
        r#"#!/bin/sh
COUNTER='{}'
N=$(cat "$COUNTER" 2>/dev/null || echo 0)
echo $((N + 1)) > "$COUNTER"
exec python3 -c 'import sys, json; d = sys.stdin.read(); print(json.dumps({{"input": json.loads(d)}}))'
"#,
        counter_path.display()
    );
    fs::write(&bin, script).unwrap();
    let mut perms = fs::metadata(&bin).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin, perms).unwrap();
    dir
}

fn read_counter(p: &std::path::Path) -> u32 {
    fs::read_to_string(p)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

#[test]
fn cache_enabled_query_step_reuses_result_on_second_run() {
    if !python3_available() {
        eprintln!("skipping: python3 not available");
        return;
    }
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());
    let work = tempfile::tempdir().unwrap();
    let counter = work.path().join("calls.txt");
    let bin = counting_mock_bin_dir(&counter);
    let db = work.path().join("cache.sqlite");
    let dag_text = format!(
        r#"
[pipe]
cache = {{ enabled = true, ttl_sec = 3600, db = "{}" }}

[[steps]]
id = "only"
atom = "mockcrate::echo"
kind = "query"
input = {{ q = "same" }}
"#,
        db.display()
    );
    let path = write_toml(work.path(), "dag.toml", &dag_text);
    std::env::set_var("KEI_RUNTIME_BIN_DIR", bin.path());
    let r1 = run_dag(&path).expect("run1");
    let r2 = run_dag(&path).expect("run2");
    std::env::remove_var("KEI_RUNTIME_BIN_DIR");
    assert!(r1.final_ok() && r2.final_ok());
    // First run is a miss (source="fresh"), second a hit (source="cache").
    assert_eq!(r1.steps[0].source.as_deref(), Some("fresh"));
    assert_eq!(r2.steps[0].source.as_deref(), Some("cache"));
    // Atom was invoked exactly once across both runs.
    assert_eq!(read_counter(&counter), 1, "atom should have been called once");
}

#[test]
fn cache_disabled_always_invokes_atom() {
    if !python3_available() {
        eprintln!("skipping: python3 not available");
        return;
    }
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());
    let work = tempfile::tempdir().unwrap();
    let counter = work.path().join("calls.txt");
    let bin = counting_mock_bin_dir(&counter);
    let dag_text = r#"
[[steps]]
id = "only"
atom = "mockcrate::echo"
kind = "query"
input = { q = "same" }
"#;
    let path = write_toml(work.path(), "dag.toml", dag_text);
    std::env::set_var("KEI_RUNTIME_BIN_DIR", bin.path());
    let r1 = run_dag(&path).expect("r1");
    let r2 = run_dag(&path).expect("r2");
    std::env::remove_var("KEI_RUNTIME_BIN_DIR");
    assert!(r1.final_ok() && r2.final_ok());
    // No cache → source is None on both runs.
    assert!(r1.steps[0].source.is_none());
    assert!(r2.steps[0].source.is_none());
    // Atom was invoked on every run.
    assert_eq!(read_counter(&counter), 2);
}

#[test]
fn cache_command_kind_is_not_cached_even_when_enabled() {
    if !python3_available() {
        eprintln!("skipping: python3 not available");
        return;
    }
    let _guard = env_lock().lock().unwrap_or_else(|p| p.into_inner());
    let work = tempfile::tempdir().unwrap();
    let counter = work.path().join("calls.txt");
    let bin = counting_mock_bin_dir(&counter);
    let db = work.path().join("cache.sqlite");
    let dag_text = format!(
        r#"
[pipe]
cache = {{ enabled = true, ttl_sec = 3600, db = "{}" }}

[[steps]]
id = "only"
atom = "mockcrate::echo"
kind = "command"
input = {{ q = "same" }}
"#,
        db.display()
    );
    let path = write_toml(work.path(), "dag.toml", &dag_text);
    std::env::set_var("KEI_RUNTIME_BIN_DIR", bin.path());
    let r1 = run_dag(&path).expect("r1");
    let r2 = run_dag(&path).expect("r2");
    std::env::remove_var("KEI_RUNTIME_BIN_DIR");
    assert!(r1.final_ok() && r2.final_ok());
    // Cache gate: command kind → no source label on either run.
    assert!(r1.steps[0].source.is_none(), "r1 source: {:?}", r1.steps[0].source);
    assert!(r2.steps[0].source.is_none(), "r2 source: {:?}", r2.steps[0].source);
    // Atom invoked on every run because cache gate refused it.
    assert_eq!(read_counter(&counter), 2);
}
