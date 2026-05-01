//! Kimi YAML agent-spec parser tests.

use kei_skill_importer::{import, SourceFormat};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn parses_yaml_spec_name_and_description() {
    let skill = import(&fixture("kimi-agent-spec.yaml"), SourceFormat::Kimi)
        .expect("parse");
    assert_eq!(skill.name, "coder");
    assert!(skill.description.contains("subagent"),
        "desc: {}", skill.description);
    assert_eq!(skill.source_format, SourceFormat::Kimi);
}

#[test]
fn yaml_spec_tools_become_tools_required() {
    let skill = import(&fixture("kimi-agent-spec.yaml"), SourceFormat::Kimi)
        .expect("parse");
    assert!(skill.tools_required.iter().any(|t| t.contains("kimi_cli.tools.shell")),
        "tools_required: {:?}", skill.tools_required);
    assert_eq!(skill.tools_required.len(), 3);
}

#[test]
fn yaml_spec_subagents_become_tags() {
    let skill = import(&fixture("kimi-agent-spec.yaml"), SourceFormat::Kimi)
        .expect("parse");
    assert!(skill.tags.iter().any(|t| t == "extend:base"),
        "expect extend:base tag, got {:?}", skill.tags);
    assert!(skill.tags.iter().any(|t| t == "subagent:reviewer"),
        "expect subagent:reviewer tag, got {:?}", skill.tags);
}

#[test]
fn body_pulled_from_system_prompt() {
    let skill = import(&fixture("kimi-agent-spec.yaml"), SourceFormat::Kimi)
        .expect("parse");
    assert!(skill.body.contains("coder subagent"),
        "body should pull system_prompt: {}", &skill.body[..skill.body.len().min(80)]);
}

#[test]
fn auto_detection_picks_kimi_for_yaml() {
    let skill = import(&fixture("kimi-agent-spec.yaml"), SourceFormat::Auto)
        .expect("parse");
    assert_eq!(skill.source_format, SourceFormat::Kimi);
}
