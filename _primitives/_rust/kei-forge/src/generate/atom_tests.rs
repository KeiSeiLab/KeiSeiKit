//! Integration-flavoured tests for the pure-Rust atom generator.
//!
//! Uses `tempfile::TempDir` to stand up a miniature copy of the repo
//! layout and exercises `generate_atom` end-to-end without touching the
//! real filesystem. Kept in its own file so `generate.rs` stays within
//! the Constructor-Pattern 200-LOC cap.

use super::{generate_atom, rel_to_root, GenerateError};
use crate::form::ForgeRequest;
use std::fs;
use std::path::{Path, PathBuf};

fn fake_repo(tmp: &Path, crate_name: &str) -> PathBuf {
    // Replicate the five template files under `<tmp>/_templates/atom/`.
    let tdir = tmp.join("_templates/atom");
    fs::create_dir_all(tdir.join("atoms/schemas")).unwrap();
    fs::create_dir_all(tdir.join("src/atoms")).unwrap();
    fs::create_dir_all(tdir.join("tests")).unwrap();
    fs::write(
        tdir.join("atoms/__VERB__.md.template"),
        "atom: __CRATE__::__VERB__ kind=__KIND__\n__DESCRIPTION__\n",
    )
    .unwrap();
    fs::write(
        tdir.join("atoms/schemas/__VERB__-input.json.template"),
        "{\"id\":\"__CRATE__/__VERB__-input\"}",
    )
    .unwrap();
    fs::write(
        tdir.join("atoms/schemas/__VERB__-output.json.template"),
        "{\"id\":\"__CRATE__/__VERB__-output\"}",
    )
    .unwrap();
    fs::write(
        tdir.join("src/atoms/__VERB_SNAKE__.rs.template"),
        "// __CRATE_SNAKE__::__VERB_SNAKE__\n",
    )
    .unwrap();
    fs::write(
        tdir.join("tests/__VERB_SNAKE___smoke.rs.template"),
        "// test for __CRATE__::__VERB__\n",
    )
    .unwrap();

    // Replicate the empty crate dir.
    let crate_dir = tmp.join("_primitives/_rust").join(crate_name);
    fs::create_dir_all(&crate_dir).unwrap();
    crate_dir
}

fn req() -> ForgeRequest {
    ForgeRequest {
        crate_name: "kei-task".into(),
        verb: "add-dep".into(),
        kind: "command".into(),
        description: "adds a dep".into(),
    }
}

#[test]
fn happy_path_writes_five_files() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fake_repo(root, "kei-task");

    let files = generate_atom(&req(), root).expect("generate");
    assert_eq!(files.len(), 5);
    for f in &files {
        assert!(f.exists(), "missing {}", f.display());
    }

    // Placeholder substitution did happen.
    let md = fs::read_to_string(
        root.join("_primitives/_rust/kei-task/atoms/add-dep.md"),
    )
    .unwrap();
    assert!(md.contains("kei-task::add-dep"), "{md}");
    assert!(md.contains("kind=command"), "{md}");
    assert!(md.contains("adds a dep"), "{md}");

    // VERB_SNAKE flips - to _.
    let rs = fs::read_to_string(
        root.join("_primitives/_rust/kei-task/src/atoms/add_dep.rs"),
    )
    .unwrap();
    assert!(rs.contains("kei_task::add_dep"), "{rs}");
}

#[test]
fn refuses_to_overwrite_existing_file() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let crate_dir = fake_repo(root, "kei-task");

    // Pre-create one of the five target files.
    fs::create_dir_all(crate_dir.join("atoms")).unwrap();
    fs::write(crate_dir.join("atoms/add-dep.md"), "pre-existing\n").unwrap();

    let err = generate_atom(&req(), root).unwrap_err();
    assert!(matches!(err, GenerateError::FileExists(_)), "got {err:?}");

    // And nothing else got written as a side-effect.
    assert!(!crate_dir.join("src/atoms/add_dep.rs").exists());
    assert!(!crate_dir.join("tests/add_dep_smoke.rs").exists());
}

#[test]
fn errors_when_crate_dir_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Templates yes, crate no.
    let tdir = root.join("_templates/atom");
    fs::create_dir_all(tdir.join("atoms/schemas")).unwrap();
    fs::create_dir_all(tdir.join("src/atoms")).unwrap();
    fs::create_dir_all(tdir.join("tests")).unwrap();

    let err = generate_atom(&req(), root).unwrap_err();
    assert!(matches!(err, GenerateError::CrateNotFound(_)), "got {err:?}");
}

#[test]
fn errors_when_template_dir_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Crate yes, templates no.
    fs::create_dir_all(root.join("_primitives/_rust/kei-task")).unwrap();

    let err = generate_atom(&req(), root).unwrap_err();
    assert!(matches!(err, GenerateError::TemplateMissing(_)), "got {err:?}");
}

#[test]
fn forge_result_relativises_paths() {
    let root = Path::new("/tmp/fake-root");
    let abs = root.join("_primitives/_rust/kei-task/atoms/add-dep.md");
    assert_eq!(
        rel_to_root(&abs, root),
        "_primitives/_rust/kei-task/atoms/add-dep.md"
    );
}
