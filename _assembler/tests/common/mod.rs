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

/// Seed a fresh temp dir with the `_manifests/` and `_blocks/` from fixtures.
/// Returns the `TempDir` guard (keeps it alive) and the agent root path.
pub fn seed_tempdir() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().expect("mktempdir");
    let root = tmp.path().to_path_buf();
    let fx = fixtures_dir();
    copy_dir(&fx.join("_manifests"), &root.join("_manifests"));
    copy_dir(&fx.join("_blocks"), &root.join("_blocks"));
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
