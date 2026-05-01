//! Integration tests for the v0.wave14 `rule_blocks` field.
//!
//! Strategy: invoke the `assemble` binary with `KEI_REGISTRY_DB` pointing at a
//! temp SQLite DB seeded with synthetic fragment rows. Helpers live in
//! `rule_blocks_helpers/mod.rs` (separate module to keep this file ≤200 LOC).

mod common;
mod rule_blocks_helpers;

use common::{assemble_bin, read_generated};
use rule_blocks_helpers::setup_kit;
use std::path::Path;
use std::process::Command;

fn run(root: &Path, db_path: &Path, extra_args: &[&str]) -> (bool, String, String) {
    let mut cmd = Command::new(assemble_bin());
    cmd.env("AGENT_ROOT", root)
        .env("HOME", root)
        .env("KEI_REGISTRY_DB", db_path);
    for a in extra_args {
        cmd.arg(a);
    }
    let out = cmd.output().expect("spawn assemble");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

// ── tests ─────────────────────────────────────────────────────────────────

/// Fragment body appears after # ROLE and before first block.
#[test]
fn rule_blocks_injected_after_role_before_blocks() {
    let (_tmp, root, db) = setup_kit(
        &["foo::think"],
        &[("foo::think", "## Think Before Coding\n\nProactive rule text.")],
    );
    let (ok, _out, stderr) = run(&root, &db, &[]);
    assert!(ok, "assemble failed: {stderr}");

    let md = read_generated(&root, "test-rule-blocks");
    let role_pos = md.find("# ROLE").expect("# ROLE missing");
    let frag_pos = md.find("Proactive rule text.").expect("fragment body missing");
    let baseline_pos = md.find("# BASELINE").expect("# BASELINE missing");

    assert!(role_pos < frag_pos, "rule fragment must come AFTER # ROLE");
    assert!(frag_pos < baseline_pos, "rule fragment must come BEFORE first block (# BASELINE)");
}

/// `<!-- RULE: name -->` comment marker emitted for each fragment.
#[test]
fn rule_blocks_comment_marker_present() {
    let (_tmp, root, db) = setup_kit(
        &["karpathy::surgical"],
        &[("karpathy::surgical", "## Surgical Changes\n\nTouch only what you must.")],
    );
    let (ok, _out, stderr) = run(&root, &db, &[]);
    assert!(ok, "assemble failed: {stderr}");

    let md = read_generated(&root, "test-rule-blocks");
    assert!(
        md.contains("<!-- RULE: karpathy::surgical -->"),
        "missing comment marker in generated md"
    );
}

/// Multiple fragments appear in the order declared in the manifest.
#[test]
fn rule_blocks_order_preserved() {
    let (_tmp, root, db) = setup_kit(
        &["alpha::one", "beta::two"],
        &[
            ("alpha::one", "Alpha body text."),
            ("beta::two", "Beta body text."),
        ],
    );
    let (ok, _out, stderr) = run(&root, &db, &[]);
    assert!(ok, "assemble failed: {stderr}");

    let md = read_generated(&root, "test-rule-blocks");
    let alpha_pos = md.find("Alpha body text.").expect("alpha missing");
    let beta_pos = md.find("Beta body text.").expect("beta missing");
    assert!(alpha_pos < beta_pos, "alpha must appear before beta in output");
}

/// Absent registry DB → warn on stderr but assemble succeeds (graceful skip).
#[test]
fn missing_registry_db_warn_and_skip() {
    let (_tmp, root, _db) = setup_kit(&["foo::bar"], &[("foo::bar", "some text")]);
    let absent_db = root.join("does-not-exist.sqlite");
    let (ok, _out, stderr) = run(&root, &absent_db, &[]);
    assert!(
        ok,
        "assemble should succeed (warn+skip) when registry DB absent; stderr: {stderr}"
    );
    assert!(
        stderr.contains("not found") || stderr.contains("rule_blocks will be skipped"),
        "expected warning about missing DB in stderr: {stderr}"
    );
}

/// Fragment name present in manifest but missing from registry → hard fail with clear message.
#[test]
fn missing_fragment_in_db_fails_validation() {
    let (_tmp, root, db) = setup_kit(&["ghost::missing"], &[]);
    let (ok, _out, stderr) = run(&root, &db, &[]);
    assert!(!ok, "assemble should FAIL when fragment not in registry; stderr: {stderr}");
    assert!(
        stderr.contains("ghost::missing"),
        "error must name the missing fragment; stderr: {stderr}"
    );
}

/// Manifests WITHOUT `rule_blocks` produce byte-identical output on re-run.
#[test]
fn no_rule_blocks_produces_identical_output() {
    let (_tmp, root, db) = setup_kit(&[], &[]);
    let (ok1, _, e1) = run(&root, &db, &[]);
    assert!(ok1, "first run failed: {e1}");
    let first = read_generated(&root, "test-rule-blocks");

    let (ok2, _, e2) = run(&root, &db, &[]);
    assert!(ok2, "second run failed: {e2}");
    let second = read_generated(&root, "test-rule-blocks");

    assert_eq!(first, second, "output must be byte-identical on re-run");
    assert!(
        !first.contains("<!-- RULE:"),
        "no-rule-blocks manifest must not emit RULE comment markers"
    );
}

/// Re-assembling a manifest WITH `rule_blocks` is byte-identical (determinism).
#[test]
fn idempotent_reassemble() {
    let (_tmp, root, db) = setup_kit(
        &["idem::check"],
        &[("idem::check", "Idempotency rule body.")],
    );
    let (ok1, _, e1) = run(&root, &db, &[]);
    assert!(ok1, "first run failed: {e1}");
    let first = read_generated(&root, "test-rule-blocks");

    let (ok2, _, e2) = run(&root, &db, &[]);
    assert!(ok2, "second run failed: {e2}");
    let second = read_generated(&root, "test-rule-blocks");

    assert_eq!(first, second, "re-assemble must be byte-identical");
}
