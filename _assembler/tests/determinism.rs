//! Determinism + ordering tests for the assembler.
//!
//! The assembler module docstring promises:
//! > Output is deterministic: same manifest + blocks → byte-identical .md
//!
//! These tests actually verify that promise. Catches any accidental
//! `HashMap`-iteration leak, embedded timestamp, or non-stable sort.

mod common;

use common::{assemble_one, seed_tempdir};
use std::fs;

/// Same input, two runs, byte-identical output.
#[test]
fn determinism_same_input_byte_identical() {
    let (_tmp1, root1) = seed_tempdir();
    let first = assemble_one(&root1, "code-implementer");

    let (_tmp2, root2) = seed_tempdir();
    let second = assemble_one(&root2, "code-implementer");

    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "two independent runs produced different bytes"
    );
}

/// Same input, ten runs, all byte-identical. Higher chance to catch
/// hash-map iteration nondeterminism that escapes a 2-run check.
#[test]
fn determinism_ten_runs_all_identical() {
    let mut seen: Option<String> = None;
    for i in 0..10 {
        let (_tmp, root) = seed_tempdir();
        let out = assemble_one(&root, "researcher");
        match &seen {
            None => seen = Some(out),
            Some(prev) => assert_eq!(
                prev.as_bytes(),
                out.as_bytes(),
                "run {i} diverged from run 0"
            ),
        }
    }
}

/// Block ordering: the order in `manifest.blocks` defines the order
/// in the output. Reorder the blocks list → output changes, and the
/// change is localized to the block region (not to frontmatter or
/// trailing sections).
#[test]
fn block_order_controls_output_order() {
    let (_tmp, root) = seed_tempdir();

    // Baseline: default kei-researcher (baseline, evidence-grading, memory-protocol).
    let default_out = assemble_one(&root, "researcher");

    // Swap two blocks — write a modified manifest into the same tempdir.
    let manifest_src = fs::read_to_string(root.join("_manifests/researcher.toml")).unwrap();
    let swapped = manifest_src.replace(
        "blocks = [\n    \"baseline\",              # OBLIGATORY\n    \"evidence-grading\",      # OBLIGATORY\n    \"memory-protocol\",       # OBLIGATORY\n]",
        "blocks = [\n    \"baseline\",\n    \"memory-protocol\",\n    \"evidence-grading\",\n]",
    );
    assert_ne!(
        manifest_src, swapped,
        "blocks-list replacement did not match — test fixture drifted"
    );
    fs::write(root.join("_manifests/researcher.toml"), &swapped).unwrap();

    let swapped_out = assemble_one(&root, "researcher");

    // 1. Output is different.
    assert_ne!(
        default_out, swapped_out,
        "swapping block order did not change output"
    );

    // 2. Frontmatter unchanged (first `---` through the trailing `---\n\n`
    //    ends identically — compare the first 500 bytes, which cover
    //    frontmatter for all our fixtures).
    let prefix_len = default_out
        .find("# BASELINE")
        .expect("BASELINE marker missing in default output");
    assert_eq!(
        &default_out[..prefix_len],
        &swapped_out[..prefix_len],
        "frontmatter + role drifted when only blocks were reordered"
    );

    // 3. The "# DOMAIN SCOPE" marker appears in both (tail section unchanged
    //    by block reordering).
    assert!(default_out.contains("# DOMAIN SCOPE"));
    assert!(swapped_out.contains("# DOMAIN SCOPE"));
}
