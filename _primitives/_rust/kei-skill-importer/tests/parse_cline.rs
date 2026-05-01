//! Cline parser tests using real-shape `.clinerules` fixture.

use kei_skill_importer::{import, SourceFormat};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parses_description_and_paths_as_tags() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    assert!(skill.description.contains("TypeScript"),
        "desc: {}", skill.description);
    assert!(skill.tags.iter().any(|t| t.starts_with("paths:")),
        "expected paths:* tag, got {:?}", skill.tags);
}

#[test]
fn flat_skill_yields_one_phase() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    assert_eq!(skill.phases.len(), 1, "cline rules are flat");
}

#[test]
fn name_derived_from_filename() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    assert!(skill.name.contains("typescript-paths") || skill.name.contains("typescript_paths"),
        "got name: {}", skill.name);
}

#[test]
fn classifier_finds_no_atom_calls_in_pure_prose() {
    let skill = import(&fixture("cline-typescript-paths.md"), SourceFormat::Cline)
        .expect("parse");
    let total: usize = skill.phases.iter().map(|p| p.atom_calls.len()).sum();
    assert_eq!(total, 0, "cline TS rule has no commands; got {:?}",
        skill.phases.iter().map(|p| &p.atom_calls).collect::<Vec<_>>());
}
