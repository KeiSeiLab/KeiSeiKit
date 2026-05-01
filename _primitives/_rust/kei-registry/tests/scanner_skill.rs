//! Skill scanner walks `<kit>/skills/<name>/SKILL.md`.

use kei_registry::scanners::skill::SkillScanner;
use kei_registry::scanners::Scanner;
use kei_registry::BlockType;
use std::path::PathBuf;

fn fixture_kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("fake-kit")
}

#[test]
fn skill_scanner_extracts_h1_title() {
    let found = SkillScanner.scan(&fixture_kit_root()).unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].block_type, BlockType::Skill);
    assert_eq!(found[0].name, "Sample Skill", "name is H1 title");
}

#[test]
fn skill_scanner_path_is_skill_md() {
    let found = SkillScanner.scan(&fixture_kit_root()).unwrap();
    assert!(found[0].path.ends_with("SKILL.md"));
}

#[test]
fn skill_scanner_caps_is_md() {
    let found = SkillScanner.scan(&fixture_kit_root()).unwrap();
    assert_eq!(found[0].caps, "md");
}
