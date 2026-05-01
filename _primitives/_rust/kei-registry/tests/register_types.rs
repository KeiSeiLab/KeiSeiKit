//! Unit tests for per-type DNA computation and idempotency.
//!
//! Covers: BlockMdScanner, CapabilityScanner, RoleScanner — each scanner
//! returns a Found with BlockType::Atom. Idempotency: re-register → no-op.

use kei_registry::scanners::block_md::BlockMdScanner;
use kei_registry::scanners::capability::CapabilityScanner;
use kei_registry::scanners::role::RoleScanner;
use kei_registry::scanners::Scanner;
use kei_registry::{open_db, register, BlockType};
use std::path::PathBuf;
use tempfile::tempdir;

fn mini_kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("mini-kit")
}

// ── BlockMdScanner ────────────────────────────────────────────────────────

#[test]
fn block_md_scanner_finds_md_files() {
    let root = mini_kit_root();
    let found = BlockMdScanner.scan(&root).unwrap();
    assert!(!found.is_empty(), "at least one .md in _blocks/");
    assert!(found.iter().all(|f| f.block_type == BlockType::Atom));
}

#[test]
fn block_md_scanner_extracts_h1_name() {
    let root = mini_kit_root();
    let found = BlockMdScanner.scan(&root).unwrap();
    let block = found.iter().find(|f| f.name == "Mini Block");
    assert!(block.is_some(), "H1 title extracted as name");
}

#[test]
fn block_md_scanner_caps_empty() {
    let root = mini_kit_root();
    let found = BlockMdScanner.scan(&root).unwrap();
    assert!(found.iter().all(|f| f.caps.is_empty()), "caps should be empty for blocks");
}

// ── CapabilityScanner ────────────────────────────────────────────────────

#[test]
fn capability_scanner_finds_capability_tomls() {
    let root = mini_kit_root();
    let found = CapabilityScanner.scan(&root).unwrap();
    assert!(!found.is_empty(), "at least one capability.toml");
    assert!(found.iter().all(|f| f.block_type == BlockType::Atom));
}

#[test]
fn capability_scanner_extracts_name_from_toml() {
    let root = mini_kit_root();
    let found = CapabilityScanner.scan(&root).unwrap();
    let cap = found.iter().find(|f| f.name == "tools::mini-cap");
    assert!(cap.is_some(), "name from [capability].name in TOML");
}

#[test]
fn capability_scanner_caps_from_category() {
    let root = mini_kit_root();
    let found = CapabilityScanner.scan(&root).unwrap();
    let cap = found.iter().find(|f| f.name == "tools::mini-cap").unwrap();
    assert_eq!(cap.caps, "tools", "caps derived from category field");
}

// ── RoleScanner ───────────────────────────────────────────────────────────

#[test]
fn role_scanner_finds_toml_files() {
    let root = mini_kit_root();
    let found = RoleScanner.scan(&root).unwrap();
    assert!(!found.is_empty(), "at least one .toml in _roles/");
    assert!(found.iter().all(|f| f.block_type == BlockType::Atom));
}

#[test]
fn role_scanner_name_is_filename_stem() {
    let root = mini_kit_root();
    let found = RoleScanner.scan(&root).unwrap();
    let role = found.iter().find(|f| f.name == "mini-role");
    assert!(role.is_some(), "stem of .toml file is the name");
}

#[test]
fn role_scanner_caps_empty() {
    let root = mini_kit_root();
    let found = RoleScanner.scan(&root).unwrap();
    assert!(found.iter().all(|f| f.caps.is_empty()), "caps empty for roles");
}

// ── Idempotency ───────────────────────────────────────────────────────────

#[test]
fn register_block_twice_same_dna() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("reg.sqlite")).unwrap();
    let body = b"block content";
    let path = "/tmp/fixture/_blocks/idempotent.md";
    let first = register(&conn, BlockType::Atom, "idempotent", path, body, "").unwrap();
    let second = register(&conn, BlockType::Atom, "idempotent", path, body, "").unwrap();
    assert_eq!(first.dna, second.dna, "re-register must return same DNA");
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM blocks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1, "exactly one row");
}

#[test]
fn register_capability_twice_same_dna() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("reg.sqlite")).unwrap();
    let body = b"[capability]\nname = \"test::cap\"";
    let path = "/tmp/fixture/_capabilities/test/cap/capability.toml";
    let first = register(&conn, BlockType::Atom, "test::cap", path, body, "test").unwrap();
    let second = register(&conn, BlockType::Atom, "test::cap", path, body, "test").unwrap();
    assert_eq!(first.dna, second.dna);
}

#[test]
fn register_role_twice_same_dna() {
    let tmp = tempdir().unwrap();
    let conn = open_db(tmp.path().join("reg.sqlite")).unwrap();
    let body = b"[role]\nname = \"auditor\"";
    let path = "/tmp/fixture/_roles/auditor.toml";
    let first = register(&conn, BlockType::Atom, "auditor", path, body, "").unwrap();
    let second = register(&conn, BlockType::Atom, "auditor", path, body, "").unwrap();
    assert_eq!(first.dna, second.dna);
}
