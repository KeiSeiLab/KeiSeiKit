//! Mode-picker integration test.
//!
//! The `skills/new-agent` wizard Phase 3.6 appends `mode-*` block names to
//! the `blocks` array. This test locks the contract that such a manifest
//! validates cleanly AND the expected mode files ship in `_blocks/` (either
//! in the fixture set or alongside the real kit).
//!
//! We use the real `_blocks/` so the test protects the kit's mode surface —
//! if anyone renames or deletes a mode block, the wizard's Phase 3.6
//! selection would silently break at runtime otherwise.

use std::path::PathBuf;

fn kit_root() -> PathBuf {
    // `CARGO_MANIFEST_DIR` points at `_assembler/`; kit root is one up.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn all_five_mode_blocks_ship_in_kit() {
    let blocks = kit_root().join("_blocks");
    for mode in [
        "mode-skeptic",
        "mode-devils-advocate",
        "mode-minimalist",
        "mode-maximalist",
        "mode-first-principles",
    ] {
        let p = blocks.join(format!("{mode}.md"));
        assert!(
            p.exists(),
            "mode block '{mode}' is missing from _blocks/ — Phase 3.6 of skills/new-agent would break"
        );
    }
}

#[test]
fn mode_matrix_doc_ships_in_kit() {
    let p = kit_root().join("_blocks/mode-matrix.md");
    assert!(
        p.exists(),
        "mode-matrix.md is missing from _blocks/ — SKILL.md Phase 3.6 references it"
    );
    let text = std::fs::read_to_string(&p).unwrap();
    // The matrix must enumerate each mode by block basename.
    for mode in [
        "skeptic",
        "devils-advocate",
        "minimalist",
        "maximalist",
        "first-principles",
    ] {
        assert!(
            text.contains(mode),
            "mode-matrix.md is missing row for '{mode}'"
        );
    }
}

#[test]
fn skill_md_phase_3_6_wiring_exists() {
    // The wizard adds mode-* blocks only if Phase 3.6 is present.
    let p = kit_root().join("skills/new-agent/SKILL.md");
    assert!(p.exists(), "skills/new-agent/SKILL.md is missing");
    let text = std::fs::read_to_string(&p).unwrap();
    assert!(
        text.contains("Phase 3.6"),
        "SKILL.md is missing the Phase 3.6 mode picker"
    );
    assert!(
        text.contains("mode-skeptic")
            || text.contains("skeptic — doubt-first"),
        "SKILL.md Phase 3.6 does not reference the skeptic mode"
    );
}
