//! Injection order is: persona, then nearest CLAUDE.md, then nearest
//! AGENTS.md, then loaded skill. Each `=== <label> ===` header marks the
//! transition. The persona has no header (it's prefix-only).

use crate::context::types::{ContextKind, DiscoveredFile, LoadedSkill};
use crate::context::build_system_prompt;
use std::path::PathBuf;

#[test]
fn order_is_persona_claude_agents_skill() {
    // Tokens chosen to avoid any cross-substring match: PERSONA contains
    // "A-BODY" if the agent token is "A-BODY", so the section markers
    // here use distinct glyphs (☆/♦/♥/♠) that cannot collide.
    let claude = DiscoveredFile {
        path: PathBuf::from("/p/CLAUDE.md"),
        content: "MARK♦CLAUDE".into(),
        kind: ContextKind::ClaudeMd,
    };
    let agents = DiscoveredFile {
        path: PathBuf::from("/p/AGENTS.md"),
        content: "MARK♥AGENTS".into(),
        kind: ContextKind::AgentsMd,
    };
    let skill = LoadedSkill {
        name: "plan".into(),
        path: PathBuf::from("/p/.claude/skills/plan/SKILL.md"),
        body: "MARK♠SKILL".into(),
    };
    let out = build_system_prompt("MARK☆PERSONA", &[claude, agents], Some(&skill));
    let p = out.find("MARK☆PERSONA").expect("persona present");
    let c = out.find("MARK♦CLAUDE").expect("claude present");
    let a = out.find("MARK♥AGENTS").expect("agents present");
    let s = out.find("MARK♠SKILL").expect("skill present");
    assert!(p < c, "persona before claude");
    assert!(c < a, "claude before agents");
    assert!(a < s, "agents before skill");
}

#[test]
fn nearest_claude_wins_when_multiple_present() {
    let inner = DiscoveredFile {
        path: PathBuf::from("/p/inner/CLAUDE.md"),
        content: "INNER-CLAUDE".into(),
        kind: ContextKind::ClaudeMd,
    };
    let outer = DiscoveredFile {
        path: PathBuf::from("/p/CLAUDE.md"),
        content: "OUTER-CLAUDE".into(),
        kind: ContextKind::ClaudeMd,
    };
    // Discover invariant: nearest first.
    let out = build_system_prompt("P", &[inner, outer], None);
    assert!(out.contains("INNER-CLAUDE"));
    assert!(!out.contains("OUTER-CLAUDE"), "only nearest should inject");
}

#[test]
fn agents_skipped_when_same_path_as_claude() {
    // Symlink scenario: CLAUDE.md and AGENTS.md resolve to same path.
    let claude = DiscoveredFile {
        path: PathBuf::from("/p/CONTEXT.md"),
        content: "BODY-ONCE".into(),
        kind: ContextKind::ClaudeMd,
    };
    let agents = DiscoveredFile {
        path: PathBuf::from("/p/CONTEXT.md"),
        content: "BODY-ONCE".into(),
        kind: ContextKind::AgentsMd,
    };
    let out = build_system_prompt("P", &[claude, agents], None);
    let occurrences = out.matches("BODY-ONCE").count();
    assert_eq!(occurrences, 1, "same path should not double-inject");
}
