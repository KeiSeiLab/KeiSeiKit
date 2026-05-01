//! Integration tests for kei-projects-index.
//!
//! Constructor Pattern: each test = one scenario, one assertion target.
//! Uses `tempfile::tempdir` for per-test isolated working trees so the
//! tests are stable on a developer machine with a populated `~/Projects/`.

use git2::{Repository, Signature};
use kei_projects_index::{
    detect_git_state, get_one, init, list_all, rebuild_index, walk_projects_root,
};
use rusqlite::Connection;
use std::fs;
use std::path::Path;

/// Init repo at `dir`, write README.md, stage + commit. Returns commit SHA.
fn make_dummy_repo(dir: &Path) -> String {
    let repo = Repository::init(dir).expect("git init");
    fs::write(dir.join("README.md"), "hello\n").expect("write readme");
    let mut index = repo.index().expect("index");
    index.add_path(Path::new("README.md")).expect("add path");
    index.write().expect("index write");
    let tree_oid = index.write_tree().expect("tree write");
    let tree = repo.find_tree(tree_oid).expect("find tree");
    let sig = Signature::now("test", "test@example.com").expect("sig");
    repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
        .expect("commit")
        .to_string()
}

#[test]
fn init_creates_schema_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("p.sqlite");
    {
        let conn = Connection::open(&db).unwrap();
        init(&conn).unwrap();
    }
    let conn = Connection::open(&db).unwrap();
    init(&conn).unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM projects", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
    let idx: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='index' AND tbl_name='projects'")
        .unwrap()
        .query_map([], |r| r.get::<_, String>(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert!(idx.iter().any(|n| n == "idx_projects_dirty"));
    assert!(idx.iter().any(|n| n == "idx_projects_last_commit"));
}

#[test]
fn rebuild_with_no_projects_returns_zero() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("p.sqlite");
    let root = dir.path().join("Projects");
    let n = rebuild_index(&db, &root).unwrap();
    assert_eq!(n, 0);
    fs::create_dir_all(&root).unwrap();
    let n2 = rebuild_index(&db, &root).unwrap();
    assert_eq!(n2, 0);
    let conn = Connection::open(&db).unwrap();
    let rows = list_all(&conn).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn rebuild_indexes_a_dummy_git_repo() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("p.sqlite");
    let root = dir.path().join("Projects");
    let project = root.join("dummy");
    fs::create_dir_all(&project).unwrap();
    let sha = make_dummy_repo(&project);
    fs::write(project.join("CLAUDE.md"), "doc\n").unwrap();
    fs::create_dir_all(root.join(".cache")).unwrap();
    fs::create_dir_all(root.join("_archive")).unwrap();
    let n = rebuild_index(&db, &root).unwrap();
    assert_eq!(n, 1, "hidden + _archive must be excluded");
    let conn = Connection::open(&db).unwrap();
    let rows = list_all(&conn).unwrap();
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.name, "dummy");
    assert!(row.has_git);
    assert!(row.has_claude_md);
    assert!(row.has_readme);
    assert!(!row.dirty);
    assert_eq!(row.last_commit_sha.as_deref(), Some(sha.as_str()));
    assert_eq!(row.last_commit_msg.as_deref(), Some("init"));
    let n2 = rebuild_index(&db, &root).unwrap();
    assert_eq!(n2, 1, "rebuild is idempotent");
    let one = get_one(&conn, &row.path).unwrap().unwrap();
    assert_eq!(one.path, row.path);
    let entries = walk_projects_root(&root).unwrap();
    assert_eq!(entries.len(), 1);
}

#[test]
fn git_state_detects_dirty_after_write() {
    let dir = tempfile::tempdir().unwrap();
    let project = dir.path().to_path_buf();
    make_dummy_repo(&project);
    let clean = detect_git_state(&project).expect("repo detected");
    assert!(!clean.dirty, "fresh repo must be clean");
    fs::write(project.join("README.md"), "hello world\n").unwrap();
    let dirty = detect_git_state(&project).expect("repo detected");
    assert!(dirty.dirty, "modified tracked file must mark dirty");
    assert_eq!(dirty.ahead, 0);
    assert_eq!(dirty.behind, 0);
    fs::write(project.join("scratch.txt"), "ignore\n").unwrap();
    let still = detect_git_state(&project).expect("repo detected");
    assert!(still.dirty, "untracked add does not unmark dirty");
}
