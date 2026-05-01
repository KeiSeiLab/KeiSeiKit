//! Integration tests: walk + identify on known sibling fixture directories.
//!
//! A1.1 scope: walk and identify only. Trait matching (A2.1) is stubbed.

use kei_import_project::{identify_modules, walk_repo, ModuleKind};
use std::path::Path;

const BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

fn sibling(crate_name: &str) -> std::path::PathBuf {
    Path::new(BASE).join(crate_name)
}

#[test]
fn walk_kei_ping_finds_rust_crate() {
    let path = sibling("kei-ping");
    if !path.exists() {
        return; // fixture not present in this checkout
    }
    let walk = walk_repo(&path).expect("walk kei-ping");
    let modules = identify_modules(&walk).expect("identify kei-ping");
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].kind, ModuleKind::RustCrate);
    assert_eq!(modules[0].name, "kei-ping");
    assert!(!modules[0].source_files.is_empty());
}

#[test]
fn walk_kei_tlog_finds_rust_crate() {
    let path = sibling("kei-tlog");
    if !path.exists() {
        return;
    }
    let walk = walk_repo(&path).expect("walk kei-tlog");
    let modules = identify_modules(&walk).expect("identify kei-tlog");
    assert!(!modules.is_empty());
    assert!(modules.iter().any(|m| m.kind == ModuleKind::RustCrate));
}
