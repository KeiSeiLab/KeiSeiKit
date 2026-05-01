//! Shared helpers for assembler integration tests.
//!
//! Strategy: the `agent-assembler` crate is binary-only (no lib target),
//! so integration tests cannot call `assembler::assemble()` directly.
//! Instead we invoke the built `assemble` binary with a controlled
//! `AGENT_ROOT` pointing at a temp dir seeded from `tests/fixtures/`.
//!
//! This tests the FULL pipeline (main.rs I/O + manifest parse +
//! validator + assembler), which is exactly the contract we want locked.

#![allow(dead_code)] // helpers used across multiple test files

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

/// Path to the fixtures directory (checked into the repo, read-only at runtime).
pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// Path to the `assemble` binary built by cargo for this test run.
/// `CARGO_BIN_EXE_<name>` is injected by cargo for integration tests.
pub fn assemble_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_assemble"))
}

/// Path to the kit root (parent of `_assembler/`). Used to source
/// `_roles/` and `_capabilities/` which are SSoT in the kit and not
/// duplicated as fixtures.
pub fn kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Seed a fresh temp dir with `_manifests/` + `_blocks/` from fixtures
/// AND `_roles/` + `_capabilities/` from the live kit root. Returns the
/// `TempDir` guard (keeps it alive) and the agent root path.
///
/// Substrate-aware manifests (those with `substrate_role`) need _roles/
/// and _capabilities/ to validate; we don't duplicate those into fixtures
/// because they're a single source of truth.
pub fn seed_tempdir() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().expect("mktempdir");
    let root = tmp.path().to_path_buf();
    let fx = fixtures_dir();
    let kit = kit_root();
    copy_dir(&fx.join("_manifests"), &root.join("_manifests"));
    copy_dir(&fx.join("_blocks"), &root.join("_blocks"));
    copy_dir(&kit.join("_roles"), &root.join("_roles"));
    copy_caps(&kit.join("_capabilities"), &root.join("_capabilities"));
    (tmp, root)
}

/// Recursive copy of a flat directory (no subdirs expected in fixtures).
pub fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir dst");
    for entry in fs::read_dir(from).expect("read src dir").flatten() {
        let src = entry.path();
        if src.is_file() {
            let dst = to.join(src.file_name().unwrap());
            fs::copy(&src, &dst).expect("copy file");
        }
    }
}

/// Two-level recursive copy: `_capabilities/<cat>/<slug>/text.md`. Used
/// only for the capabilities tree which has a fixed two-level structure.
pub fn copy_caps(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir caps root");
    for cat in fs::read_dir(from).expect("read caps").flatten() {
        let cat_path = cat.path();
        if !cat_path.is_dir() {
            continue;
        }
        let cat_dst = to.join(cat_path.file_name().unwrap());
        fs::create_dir_all(&cat_dst).expect("mkdir cat");
        for slug in fs::read_dir(&cat_path).expect("read cat").flatten() {
            let slug_path = slug.path();
            if !slug_path.is_dir() {
                continue;
            }
            let slug_dst = cat_dst.join(slug_path.file_name().unwrap());
            fs::create_dir_all(&slug_dst).expect("mkdir slug");
            for file in fs::read_dir(&slug_path).expect("read slug").flatten() {
                let fp = file.path();
                if fp.is_file() {
                    fs::copy(&fp, slug_dst.join(fp.file_name().unwrap()))
                        .expect("copy cap");
                }
            }
        }
    }
}

/// Run `assemble` with `AGENT_ROOT=<root>` and the given extra args.
/// Returns the raw `Output` for the caller to inspect stdout/stderr/status.
pub fn run_assemble(root: &Path, args: &[&str]) -> Output {
    Command::new(assemble_bin())
        .env("AGENT_ROOT", root)
        // Unset HOME-derived fallbacks so a stray HOME cannot leak into the
        // test (binary prefers AGENT_ROOT, but defence-in-depth is cheap).
        .env("HOME", root)
        .args(args)
        .output()
        .expect("spawn assemble")
}

/// Run `assemble` with no positional args (process every manifest in
/// `<root>/_manifests/`) and return the output.
pub fn run_assemble_all(root: &Path) -> Output {
    run_assemble(root, &[])
}

/// Read the generated `.md` for `<name>` under `<root>/_generated/`.
pub fn read_generated(root: &Path, name: &str) -> String {
    let p = root.join("_generated").join(format!("{name}.md"));
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

/// Assemble a single manifest end-to-end and return its generated content.
/// Panics with stderr if the binary exits non-zero.
pub fn assemble_one(root: &Path, manifest_name: &str) -> String {
    let manifest = root
        .join("_manifests")
        .join(format!("{manifest_name}.toml"));
    let out = run_assemble(root, &[manifest.to_str().unwrap()]);
    assert!(
        out.status.success(),
        "assemble {manifest_name} failed: stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    read_generated(root, manifest_name)
}
