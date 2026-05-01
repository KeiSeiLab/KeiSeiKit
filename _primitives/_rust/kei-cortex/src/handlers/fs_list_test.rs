//! Inline unit tests for `fs_list.rs`. Exercises the resolver and the
//! noise filter against a tempdir; no daemon spin-up needed.

use super::*;
use std::fs;
use tempfile::TempDir;

/// Build a tempdir with a known shape:
///   <root>/src/main.rs
///   <root>/README.md
///   <root>/.git/  (hidden)
///   <root>/node_modules/foo  (hidden)
///   <root>/_archive/old.txt  (hidden)
fn fixture() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let r = tmp.path();
    fs::create_dir_all(r.join("src")).unwrap();
    fs::write(r.join("src/main.rs"), b"fn main(){}").unwrap();
    fs::write(r.join("README.md"), b"# r").unwrap();
    fs::create_dir_all(r.join(".git")).unwrap();
    fs::create_dir_all(r.join("node_modules/foo")).unwrap();
    fs::create_dir_all(r.join("_archive")).unwrap();
    fs::write(r.join("_archive/old.txt"), b"old").unwrap();
    tmp
}

#[test]
fn lists_only_visible_root_entries() {
    let tmp = fixture();
    let entries = read_dir_entries(tmp.path()).unwrap();
    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    assert!(names.contains(&"src".to_string()));
    assert!(names.contains(&"README.md".to_string()));
    assert!(!names.contains(&".git".to_string()));
    assert!(!names.contains(&"node_modules".to_string()));
    assert!(!names.contains(&"_archive".to_string()));
}

#[test]
fn dirs_sort_before_files() {
    let tmp = fixture();
    let entries = read_dir_entries(tmp.path()).unwrap();
    let resp = build_response(entries);
    let kinds: Vec<&'static str> = resp.entries.iter().map(|e| e.kind).collect();
    let first_file = kinds.iter().position(|k| *k == "file").unwrap_or(0);
    let last_dir = kinds.iter().rposition(|k| *k == "dir").unwrap_or(usize::MAX);
    assert!(first_file > last_dir, "all dirs must precede all files");
}

#[test]
fn parent_traversal_blocked() {
    let tmp = fixture();
    let err = resolve_target(tmp.path(), "../escape").unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[test]
fn absolute_outside_root_blocked() {
    let tmp = fixture();
    let err = resolve_target(tmp.path(), "/etc").unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_) | AppError::NotFound(_)));
}

#[test]
fn empty_path_resolves_to_root() {
    let tmp = fixture();
    let p = resolve_target(tmp.path(), "").unwrap();
    let canon = tmp.path().canonicalize().unwrap();
    assert_eq!(p, canon);
}

#[test]
fn relative_path_resolves_inside_root() {
    let tmp = fixture();
    let p = resolve_target(tmp.path(), "src").unwrap();
    assert!(p.ends_with("src"));
}

#[test]
fn nonexistent_path_yields_not_found() {
    let tmp = fixture();
    let err = resolve_target(tmp.path(), "no-such-dir").unwrap_err();
    assert!(matches!(err, AppError::NotFound(_)));
}

#[test]
fn file_target_is_rejected() {
    let tmp = fixture();
    let err = resolve_target(tmp.path(), "README.md").unwrap_err();
    assert!(matches!(err, AppError::BadRequest(_)));
}

#[test]
fn should_hide_filters_noise() {
    assert!(should_hide("node_modules"));
    assert!(should_hide(".git"));
    assert!(should_hide(".env"));
    assert!(should_hide("target"));
    assert!(!should_hide("src"));
    assert!(!should_hide("README.md"));
}

#[test]
fn should_hide_blocks_separator_anchored_variants() {
    // MISS-3: `node_modules.bak`, `target-archive`, `dist_old` are noise too.
    assert!(should_hide("node_modules.bak"));
    assert!(should_hide("node_modules-archive"));
    assert!(should_hide("node_modules_extra"));
    assert!(should_hide("target-archive"));
    assert!(should_hide("dist_old"));
    assert!(should_hide(".svelte-kit.bak"));
}

#[test]
fn should_hide_does_not_overshoot_legit_names() {
    // Names sharing a prefix without a separator stay visible.
    assert!(!should_hide("nodejs"));
    assert!(!should_hide("targets"));
    assert!(!should_hide("distance"));
    assert!(!should_hide("cachelib"));
}

#[test]
fn entry_to_fs_entry_marks_size_for_files_only() {
    let tmp = fixture();
    for entry in fs::read_dir(tmp.path()).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(e) = entry_to_fs_entry(&entry, &name) {
            if e.kind == "dir" {
                assert!(e.size.is_none(), "dirs must not carry size");
            } else if e.kind == "file" {
                assert!(e.size.is_some(), "files must carry size");
            }
        }
    }
}
