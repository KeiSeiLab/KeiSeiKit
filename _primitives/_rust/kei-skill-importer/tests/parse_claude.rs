//! Claude Code SKILL.md parser tests.

use kei_skill_importer::{import, AtomCallKind, SourceFormat};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parses_name_description_and_category() {
    let skill = import(&fixture("claude-pet-init.md"), SourceFormat::ClaudeCode)
        .expect("parse");
    assert_eq!(skill.name, "pet-init");
    assert!(skill.description.contains("AI pet"));
    assert!(skill.tags.iter().any(|t| t == "category:pet"),
        "expect category:pet tag, got {:?}", skill.tags);
}

#[test]
fn classifier_resolves_kei_pet_keygen_and_validate() {
    let skill = import(&fixture("claude-pet-init.md"), SourceFormat::ClaudeCode)
        .expect("parse");
    // Find any phase that contains kei-pet calls
    let kei_pet_calls: Vec<_> = skill
        .phases
        .iter()
        .flat_map(|p| p.atom_calls.iter())
        .filter(|c| c.raw_command.starts_with("kei-pet"))
        .collect();
    assert!(kei_pet_calls.len() >= 2,
        "expect ≥2 kei-pet calls, got {:?}", kei_pet_calls);
    let keygen = kei_pet_calls.iter()
        .find(|c| c.raw_command.starts_with("kei-pet keygen"))
        .expect("keygen");
    assert_eq!(keygen.atom_id.as_deref(), Some("kei-pet::keygen"));
    assert_eq!(keygen.kind, AtomCallKind::KeiPrimitive);
}

#[test]
fn slash_command_detected() {
    let skill = import(&fixture("claude-pet-init.md"), SourceFormat::ClaudeCode)
        .expect("parse");
    let slashes: Vec<_> = skill
        .phases
        .iter()
        .flat_map(|p| p.atom_calls.iter())
        .filter(|c| c.kind == AtomCallKind::UserPrompt)
        .collect();
    assert!(slashes.iter().any(|c| c.raw_command == "/escalate-recurrence"),
        "expect /escalate-recurrence slash, got {:?}", slashes);
}

#[test]
fn h2_sections_become_phases() {
    let skill = import(&fixture("claude-pet-init.md"), SourceFormat::ClaudeCode)
        .expect("parse");
    let names: Vec<&str> = skill.phases.iter().map(|p| p.name.as_str()).collect();
    assert!(names.iter().any(|n| n.starts_with("Phase 1")),
        "expect 'Phase 1' section, got {:?}", names);
    assert!(names.iter().any(|n| n.starts_with("Phase 4")),
        "expect 'Phase 4' section, got {:?}", names);
}
