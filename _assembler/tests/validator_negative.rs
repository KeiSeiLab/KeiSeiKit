//! Validator negative-path tests.
//!
//! Locks the error contract of validator.rs: each flavour of bad
//! manifest produces a non-zero exit status AND a stderr message
//! that names the offending invariant.
//!
//! Note: the unsubstituted-`{{placeholder}}` check is being added
//! in a parallel PR (fix/remaining-findings). That specific test
//! is deliberately NOT included here; when the check lands, add a
//! case here and re-run.

mod common;

use common::{run_assemble, seed_tempdir};
use std::fs;
use std::path::Path;

/// Write a minimal valid manifest then mutate one field to break it.
/// Returns the tempdir guard (keeps it alive) and the manifest path.
fn write_broken(
    root: &Path,
    filename: &str,
    mutate: impl FnOnce(&mut String),
) -> std::path::PathBuf {
    let src = fs::read_to_string(root.join("_manifests/researcher.toml")).unwrap();
    let mut buf = src;
    mutate(&mut buf);
    let target = root.join("_manifests").join(filename);
    fs::write(&target, buf).unwrap();
    target
}

fn assert_fails_with(root: &Path, manifest: &Path, needle: &str) {
    let out = run_assemble(root, &[manifest.to_str().unwrap()]);
    assert!(
        !out.status.success(),
        "expected non-zero exit for broken manifest {}; stdout={:?} stderr={:?}",
        manifest.display(),
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        combined.contains(needle),
        "stderr did not mention {needle:?}; full output:\n{combined}"
    );
}

#[test]
fn validator_rejects_unknown_block_ref() {
    let (_tmp, root) = seed_tempdir();
    // Add an extra block name that doesn't exist on disk.
    let manifest = write_broken(&root, "broken-unknown-block.toml", |s| {
        *s = s.replace(
            "\"memory-protocol\",       # OBLIGATORY\n]",
            "\"memory-protocol\",\n    \"this-block-does-not-exist\",\n]",
        );
    });
    assert_fails_with(&root, &manifest, "this-block-does-not-exist");
}

#[test]
fn validator_rejects_missing_obligatory_block() {
    let (_tmp, root) = seed_tempdir();
    // Drop "memory-protocol" from the blocks list.
    let manifest = write_broken(&root, "broken-missing-obligatory.toml", |s| {
        *s = s.replace("\"memory-protocol\",       # OBLIGATORY\n", "");
    });
    assert_fails_with(&root, &manifest, "memory-protocol");
}

#[test]
fn validator_rejects_empty_handoff() {
    let (_tmp, root) = seed_tempdir();
    // Strip every `[[handoff]]` table from the manifest.
    let manifest = write_broken(&root, "broken-no-handoff.toml", |s| {
        let mut out = String::new();
        let mut skip = false;
        for line in s.lines() {
            if line.trim_start().starts_with("[[handoff]]") {
                skip = true;
                continue;
            }
            if skip && (line.trim_start().starts_with("[") || line.trim().is_empty()) {
                // End of the handoff block (next [table] or blank-line gap).
                if line.trim_start().starts_with("[") && !line.trim_start().starts_with("[[handoff]]") {
                    skip = false;
                } else if line.trim().is_empty() {
                    // Tolerate blank line inside handoff table separator.
                    continue;
                }
            }
            if !skip {
                out.push_str(line);
                out.push('\n');
            }
        }
        *s = out;
    });
    assert_fails_with(&root, &manifest, "handoff");
}

#[test]
fn validator_rejects_empty_role() {
    let (_tmp, root) = seed_tempdir();
    // Replace the role with whitespace only.
    let manifest = write_broken(&root, "broken-empty-role.toml", |s| {
        // The kei-researcher manifest uses triple-quoted `role = """..."""`.
        let start = s.find("role = \"\"\"").expect("role block marker missing");
        let end_rel = s[start..]
            .find("\"\"\"\n")
            .and_then(|_| s[start + 10..].find("\"\"\""))
            .expect("role closing marker missing");
        let end = start + 10 + end_rel + 3;
        let before = &s[..start];
        let after = &s[end..];
        *s = format!("{before}role = \"   \"\n{after}");
    });
    assert_fails_with(&root, &manifest, "role");
}

#[test]
fn validator_rejects_empty_domain_in() {
    let (_tmp, root) = seed_tempdir();
    // Replace domain_in array with an empty one.
    let manifest = write_broken(&root, "broken-empty-domain-in.toml", |s| {
        let start = s.find("domain_in = [").expect("domain_in marker missing");
        let end_rel = s[start..].find("]\n").expect("domain_in close marker missing");
        let end = start + end_rel + 2;
        let before = &s[..start];
        let after = &s[end..];
        *s = format!("{before}domain_in = []\n{after}");
    });
    assert_fails_with(&root, &manifest, "domain_in");
}

#[test]
fn validate_only_flag_skips_write() {
    // --validate must NOT write anything under _generated/.
    let (_tmp, root) = seed_tempdir();
    let manifest = root.join("_manifests/researcher.toml");
    let out = run_assemble(&root, &["--validate", manifest.to_str().unwrap()]);
    assert!(
        out.status.success(),
        "--validate on a valid manifest failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let generated = root.join("_generated/researcher.md");
    assert!(
        !generated.exists(),
        "--validate wrote an output file at {}",
        generated.display()
    );
}
