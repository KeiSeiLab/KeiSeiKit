//! Integration tests for registry_writer: register, idempotency, supersede.
//!
//! Uses tempfile for ephemeral SQLite + ephemeral repo trees. No live I/O.

use kei_import_project::{identify_modules, register_modules, walk_repo};
use kei_registry::{open_db, list_by_type, BlockType};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

// ── helpers ──────────────────────────────────────────────────────────────────

fn mk(dir: &Path, rel: &str, content: &str) {
    let p = dir.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, content).unwrap();
}

/// Build a synthetic Rust mono-repo with 3 named crates.
fn synthetic_repo(root: &Path) {
    mk(root, "Cargo.toml", "[workspace]\nmembers = [\"alpha\",\"beta\",\"gamma\"]\n");
    mk(root, "alpha/Cargo.toml", "[package]\nname = \"alpha\"\nversion = \"0.1.0\"\n");
    mk(root, "alpha/src/lib.rs", "pub fn alpha() {}");
    mk(root, "beta/Cargo.toml", "[package]\nname = \"beta\"\nversion = \"0.1.0\"\n");
    mk(root, "beta/src/lib.rs", "pub fn beta() {}");
    mk(root, "gamma/Cargo.toml", "[package]\nname = \"gamma\"\nversion = \"0.1.0\"\n");
    mk(root, "gamma/src/lib.rs", "pub fn gamma() {}");
}

fn modules_for(root: &Path) -> Vec<kei_import_project::ProjectModule> {
    let walk = walk_repo(root).unwrap();
    identify_modules(&walk).unwrap()
}

// ── tests ────────────────────────────────────────────────────────────────────

#[test]
fn three_modules_register_as_primitives() {
    let repo = TempDir::new().unwrap();
    synthetic_repo(repo.path());

    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let modules = modules_for(repo.path());
    assert_eq!(modules.len(), 3, "expected 3 crates");

    let result = register_modules(&modules, repo.path(), Some(&db_path)).unwrap();
    assert_eq!(result.registered, 3);
    assert_eq!(result.superseded, 0);
    assert_eq!(result.unchanged, 0);

    // Verify rows in DB.
    let conn = open_db(&db_path).unwrap();
    let rows = list_by_type(&conn, BlockType::Primitive).unwrap();
    assert_eq!(rows.len(), 3);
    assert!(rows.iter().all(|b| b.block_type == BlockType::Primitive));
}

#[test]
fn re_register_same_content_is_unchanged() {
    let repo = TempDir::new().unwrap();
    synthetic_repo(repo.path());
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let modules = modules_for(repo.path());

    // First pass.
    register_modules(&modules, repo.path(), Some(&db_path)).unwrap();
    // Second pass — same content.
    let result = register_modules(&modules, repo.path(), Some(&db_path)).unwrap();

    assert_eq!(result.registered, 0);
    assert_eq!(result.superseded, 0);
    assert_eq!(result.unchanged, 3);
}

#[test]
fn modified_source_yields_one_superseded() {
    let repo = TempDir::new().unwrap();
    synthetic_repo(repo.path());
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let modules = modules_for(repo.path());
    register_modules(&modules, repo.path(), Some(&db_path)).unwrap();

    // Modify alpha's source.
    mk(repo.path(), "alpha/src/lib.rs", "pub fn alpha_v2() {}");

    let modules2 = modules_for(repo.path());
    let result = register_modules(&modules2, repo.path(), Some(&db_path)).unwrap();

    assert_eq!(result.superseded, 1, "alpha changed → 1 superseded");
    assert_eq!(result.unchanged, 2, "beta + gamma unchanged");
    assert_eq!(result.registered, 0);
}

#[test]
fn dna_wire_format_is_valid() {
    let repo = TempDir::new().unwrap();
    synthetic_repo(repo.path());
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let modules = modules_for(repo.path());
    register_modules(&modules, repo.path(), Some(&db_path)).unwrap();

    let conn = open_db(&db_path).unwrap();
    let rows = list_by_type(&conn, BlockType::Primitive).unwrap();
    for row in &rows {
        // DNA: <role>::<caps>::<scope8>::<body8>-<nonce8>
        let parts: Vec<&str> = row.dna.split("::").collect();
        assert_eq!(parts.len(), 4, "DNA must have 4 '::'-separated segments: {}", row.dna);
        assert_eq!(parts[0], "primitive");
        // tail = <body8>-<nonce8>
        let tail = parts[3];
        let (body, nonce) = tail.split_once('-').expect("DNA tail missing '-'");
        assert_eq!(body.len(), 8, "body_sha must be 8 hex chars");
        assert_eq!(nonce.len(), 8, "nonce must be 8 hex chars");
        assert!(body.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(nonce.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

#[test]
fn path_column_stores_absolute_manifest_path() {
    let repo = TempDir::new().unwrap();
    synthetic_repo(repo.path());
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let modules = modules_for(repo.path());
    register_modules(&modules, repo.path(), Some(&db_path)).unwrap();

    let conn = open_db(&db_path).unwrap();
    let rows = list_by_type(&conn, BlockType::Primitive).unwrap();
    for row in &rows {
        let p = std::path::Path::new(&row.path);
        assert!(p.is_absolute(), "path must be absolute: {}", row.path);
        assert!(row.path.ends_with("Cargo.toml"), "path must point to manifest: {}", row.path);
    }
}

#[test]
fn name_column_contains_project_slug_prefix() {
    let repo = TempDir::new().unwrap();
    synthetic_repo(repo.path());
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let slug = kei_import_project::project_slug(repo.path());
    let modules = modules_for(repo.path());
    register_modules(&modules, repo.path(), Some(&db_path)).unwrap();

    let conn = open_db(&db_path).unwrap();
    let rows = list_by_type(&conn, BlockType::Primitive).unwrap();
    for row in &rows {
        assert!(
            row.name.starts_with(&format!("{slug}::")),
            "name must start with '{slug}::': {}",
            row.name
        );
    }
}

#[test]
fn empty_repo_registers_zero_rows() {
    let repo = TempDir::new().unwrap();
    // No manifests — just a random file.
    mk(repo.path(), "README.md", "# empty");
    let db_dir = TempDir::new().unwrap();
    let db_path = db_dir.path().join("test.sqlite");

    let modules = modules_for(repo.path());
    assert_eq!(modules.len(), 0);

    let result = register_modules(&modules, repo.path(), Some(&db_path)).unwrap();
    assert_eq!(result.registered, 0);
    assert_eq!(result.superseded, 0);
    assert_eq!(result.unchanged, 0);
}
