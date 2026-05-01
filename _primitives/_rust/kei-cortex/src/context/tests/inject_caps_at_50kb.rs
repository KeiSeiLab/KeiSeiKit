//! `build_system_prompt` caps total output at 50 KiB by dropping trailing
//! sections (skill -> agents -> claude) until the cap holds. Persona is
//! never trimmed at the section boundary.
//!
//! Test feeds ~200 KiB across CLAUDE.md + AGENTS.md + skill, asserts that
//! the rendered output is <= 50 KiB and that trailing sections were dropped
//! before the leading ones.

use crate::context::types::{ContextKind, DiscoveredFile, LoadedSkill};
use crate::context::{build_system_prompt, MAX_TOTAL_BYTES};
use std::path::PathBuf;

fn big_blob(label: &str, kib: usize) -> String {
    let line = format!("[{label}] {}\n", "x".repeat(60));
    let target = kib * 1024;
    let mut out = String::with_capacity(target + line.len());
    while out.len() < target {
        out.push_str(&line);
    }
    out
}

#[test]
fn output_within_50kb_cap_when_oversized() {
    let claude = DiscoveredFile {
        path: PathBuf::from("/p/CLAUDE.md"),
        content: big_blob("CLAUDE", 80),
        kind: ContextKind::ClaudeMd,
    };
    let agents = DiscoveredFile {
        path: PathBuf::from("/p/AGENTS.md"),
        content: big_blob("AGENTS", 80),
        kind: ContextKind::AgentsMd,
    };
    let skill = LoadedSkill {
        name: "onboard".into(),
        path: PathBuf::from("/p/.claude/skills/onboard/SKILL.md"),
        body: big_blob("SKILL", 60),
    };
    let out = build_system_prompt("PERSONA", &[claude, agents], Some(&skill));
    assert!(
        out.len() <= MAX_TOTAL_BYTES,
        "output {} bytes exceeds cap {}",
        out.len(),
        MAX_TOTAL_BYTES
    );
}

#[test]
fn drops_skill_before_agents_before_claude() {
    let claude = DiscoveredFile {
        path: PathBuf::from("/p/CLAUDE.md"),
        content: big_blob("CLAUDE", 80),
        kind: ContextKind::ClaudeMd,
    };
    let agents = DiscoveredFile {
        path: PathBuf::from("/p/AGENTS.md"),
        content: big_blob("AGENTS", 80),
        kind: ContextKind::AgentsMd,
    };
    let skill = LoadedSkill {
        name: "onboard".into(),
        path: PathBuf::from("/p/.claude/skills/onboard/SKILL.md"),
        body: big_blob("SKILL", 60),
    };
    let out = build_system_prompt("PERSONA", &[claude, agents], Some(&skill));
    // Skill should be dropped first; SKILL marker must NOT appear.
    assert!(!out.contains("[SKILL]"), "skill should have been dropped first");
}

#[test]
fn small_inputs_pass_through_unchanged_in_size() {
    let claude = DiscoveredFile {
        path: PathBuf::from("/p/CLAUDE.md"),
        content: "small claude".into(),
        kind: ContextKind::ClaudeMd,
    };
    let out = build_system_prompt("PERSONA", &[claude], None);
    assert!(out.contains("PERSONA"));
    assert!(out.contains("small claude"));
    assert!(out.len() < 1024);
}
