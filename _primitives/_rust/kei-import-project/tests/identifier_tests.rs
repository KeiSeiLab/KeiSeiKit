//! Identifier unit tests — extracted from src/identifier.rs to keep LOC ≤200.

use kei_import_project::{identify_modules, walk_repo, ModuleKind};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn mk(dir: &Path, rel: &str, content: &str) {
    let p = dir.join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, content).unwrap();
}

#[test]
fn single_rust_crate() {
    let tmp = TempDir::new().unwrap();
    let r = tmp.path();
    mk(r, "Cargo.toml", "[package]\nname = \"my-crate\"\nversion = \"0.1.0\"\nedition = \"2021\"\n");
    mk(r, "src/lib.rs", "pub fn foo() {}");
    let walk = walk_repo(r).unwrap();
    let modules = identify_modules(&walk).unwrap();
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].kind, ModuleKind::RustCrate);
    assert_eq!(modules[0].name, "my-crate");
    assert_eq!(modules[0].source_files.len(), 1);
}

#[test]
fn single_npm_package() {
    let tmp = TempDir::new().unwrap();
    let r = tmp.path();
    mk(r, "package.json", r#"{"name":"my-app","version":"1.0.0"}"#);
    mk(r, "index.ts", "export const x = 1;");
    let walk = walk_repo(r).unwrap();
    let modules = identify_modules(&walk).unwrap();
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].kind, ModuleKind::NpmPackage);
    assert_eq!(modules[0].name, "my-app");
}

#[test]
fn cargo_workspace_two_members() {
    let tmp = TempDir::new().unwrap();
    let r = tmp.path();
    mk(r, "crate-a/Cargo.toml", "[package]\nname = \"crate-a\"\nversion = \"0.1.0\"\nedition = \"2021\"\n");
    mk(r, "crate-a/src/lib.rs", "");
    mk(r, "crate-b/Cargo.toml", "[package]\nname = \"crate-b\"\nversion = \"0.1.0\"\nedition = \"2021\"\n");
    mk(r, "crate-b/src/lib.rs", "");
    let walk = walk_repo(r).unwrap();
    let modules = identify_modules(&walk).unwrap();
    assert_eq!(modules.len(), 2);
    let names: Vec<_> = modules.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"crate-a"));
    assert!(names.contains(&"crate-b"));
}

#[test]
fn mixed_monorepo_three_modules() {
    let tmp = TempDir::new().unwrap();
    let r = tmp.path();
    mk(r, "rust-pkg/Cargo.toml", "[package]\nname = \"rust-pkg\"\nversion = \"0.1.0\"\nedition = \"2021\"\n");
    mk(r, "rust-pkg/src/lib.rs", "");
    mk(r, "py-pkg/pyproject.toml", "[project]\nname = \"py-pkg\"\n");
    mk(r, "py-pkg/main.py", "print('hi')");
    mk(r, "ts-pkg/package.json", r#"{"name":"ts-pkg","version":"0.0.1"}"#);
    mk(r, "ts-pkg/index.ts", "export {};");
    let walk = walk_repo(r).unwrap();
    let modules = identify_modules(&walk).unwrap();
    assert_eq!(modules.len(), 3);
}

#[test]
fn malformed_cargo_toml_returns_err() {
    let tmp = TempDir::new().unwrap();
    let r = tmp.path();
    mk(r, "Cargo.toml", "this is not valid toml [[[");
    mk(r, "src/lib.rs", "");
    let walk = walk_repo(r).unwrap();
    assert!(identify_modules(&walk).is_err());
}

#[test]
fn empty_repo_returns_empty_vec() {
    let tmp = TempDir::new().unwrap();
    let walk = walk_repo(tmp.path()).unwrap();
    assert!(identify_modules(&walk).unwrap().is_empty());
}
