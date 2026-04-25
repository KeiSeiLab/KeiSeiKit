//! OpenClaw parser tests using real-shape AGENTS.md fixture.

use kei_skill_importer::{import, SourceFormat};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parses_name_and_description() {
    let skill = import(&fixture("openclaw-create-npm.md"), SourceFormat::OpenClaw)
        .expect("parse");
    assert_eq!(skill.name, "create-npm-package");
    assert!(skill.description.contains("npm package"),
        "desc should mention npm: {}", skill.description);
    assert_eq!(skill.source_format, SourceFormat::OpenClaw);
}

#[test]
fn parses_tags_and_tools() {
    let skill = import(&fixture("openclaw-create-npm.md"), SourceFormat::OpenClaw)
        .expect("parse");
    assert!(skill.tags.contains(&"npm".to_string()));
    assert!(skill.tags.contains(&"typescript".to_string()));
    assert!(skill.tools_required.contains(&"pnpm".to_string()));
    assert!(skill.tools_required.contains(&"git".to_string()));
}

#[test]
fn splits_h2_sections_into_phases() {
    let skill = import(&fixture("openclaw-create-npm.md"), SourceFormat::OpenClaw)
        .expect("parse");
    let phase_names: Vec<&str> = skill.phases.iter().map(|p| p.name.as_str()).collect();
    assert!(phase_names.contains(&"Start"), "got {:?}", phase_names);
    assert!(phase_names.contains(&"Commands"), "got {:?}", phase_names);
    assert!(phase_names.contains(&"Code"), "got {:?}", phase_names);
    assert!(phase_names.contains(&"Gates"), "got {:?}", phase_names);
}

#[test]
fn classifier_finds_pnpm_and_kei_calls_in_commands_phase() {
    let skill = import(&fixture("openclaw-create-npm.md"), SourceFormat::OpenClaw)
        .expect("parse");
    let cmd_phase = skill
        .phases
        .iter()
        .find(|p| p.name == "Commands")
        .expect("Commands phase");
    let raws: Vec<&str> = cmd_phase
        .atom_calls
        .iter()
        .map(|c| c.raw_command.as_str())
        .collect();
    assert!(raws.iter().any(|r| r.starts_with("pnpm init")),
        "expect pnpm init bash call: {:?}", raws);
    assert!(raws.iter().any(|r| r.starts_with("kei-task create")),
        "expect kei-task create bash call: {:?}", raws);
    let resolved_kei = cmd_phase.atom_calls.iter()
        .find(|c| c.raw_command.starts_with("kei-task create"))
        .expect("kei-task create");
    assert_eq!(resolved_kei.atom_id.as_deref(), Some("kei-task::create"));
}

#[test]
fn detects_english_language() {
    let skill = import(&fixture("openclaw-create-npm.md"), SourceFormat::OpenClaw)
        .expect("parse");
    assert_eq!(skill.language.as_deref(), Some("en"));
}
