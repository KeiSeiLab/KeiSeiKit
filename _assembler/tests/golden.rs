//! Golden-file snapshot tests for the assembler.
//!
//! Contract under test: `same manifest + blocks → byte-identical .md`
//! (assembler.rs:2). This file locks the generated output for 3
//! representative manifests:
//!
//! - `kei-researcher`        — minimal (only obligatory blocks)
//! - `kei-cost-guardian`     — minimal + output_extra_fields
//! - `kei-code-implementer`  — obligatory + 4 implementer blocks
//!
//! First run generates `tests/snapshots/*.snap.new`; approve with
//! `cargo insta review`. Subsequent runs assert byte-equality against
//! the approved snapshot. Any drift in assembler output will fail loudly.

mod common;

use common::{assemble_one, seed_tempdir};

/// Point insta at `tests/snapshots/` (not the default
/// `tests/snapshots/` inside each test binary) and use our own stable
/// snapshot naming scheme.
fn insta_settings() -> insta::Settings {
    let mut s = insta::Settings::clone_current();
    s.set_snapshot_path("snapshots");
    s.set_prepend_module_to_snapshot(false);
    s
}

#[test]
fn golden_researcher() {
    let (_tmp, root) = seed_tempdir();
    let out = assemble_one(&root, "researcher");
    insta_settings().bind(|| insta::assert_snapshot!("researcher", out));
}

#[test]
fn golden_cost_guardian() {
    let (_tmp, root) = seed_tempdir();
    let out = assemble_one(&root, "cost-guardian");
    insta_settings().bind(|| insta::assert_snapshot!("cost-guardian", out));
}

#[test]
fn golden_code_implementer() {
    let (_tmp, root) = seed_tempdir();
    let out = assemble_one(&root, "code-implementer");
    insta_settings().bind(|| insta::assert_snapshot!("code-implementer", out));
}
