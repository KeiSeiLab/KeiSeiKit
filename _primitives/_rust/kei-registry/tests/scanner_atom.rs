//! Atom scanner walks markdown files and filters frontmatter `type: atom`.

use kei_registry::scanners::atom::AtomScanner;
use kei_registry::scanners::Scanner;
use kei_registry::BlockType;
use std::path::PathBuf;

fn fixture_atom_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("atom-sample")
}

#[test]
fn atom_scanner_finds_atom_frontmatter_md() {
    let found = AtomScanner.scan(&fixture_atom_root()).unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].block_type, BlockType::Atom);
    assert_eq!(found[0].name, "foo", "name comes from frontmatter `name: foo`");
}

#[test]
fn atom_scanner_skips_non_atom_md() {
    // Re-scan rules fixtures (non-atom md) — should return empty.
    let rules = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("fake-claude")
        .join("rules");
    let found = AtomScanner.scan(&rules).unwrap();
    assert!(
        found.is_empty(),
        "rules .md without frontmatter type=atom must be skipped, got {found:?}"
    );
}
