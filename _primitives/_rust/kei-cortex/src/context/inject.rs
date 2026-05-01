//! Compose the final system prompt from persona + discovered context +
//! optional matched skill.
//!
//! Order (top to bottom in the rendered prompt):
//!   1. Persona system prompt (passed in by the caller; never trimmed).
//!   2. Nearest `CLAUDE.md` from `ctx` — labelled "Project context".
//!   3. Nearest `AGENTS.md` from `ctx` (only if its on-disk path differs
//!      from the CLAUDE.md path, to avoid double-injection on symlinks).
//!   4. The loaded skill body (if any) — labelled "Active skill".
//!
//! Total length cap: `MAX_TOTAL_BYTES` (50 KiB). Persona is sacred — we
//! never trim it. Beyond that, sections are dropped *oldest first* in
//! reverse-injection order: skill → AGENTS.md → CLAUDE.md, until the cap
//! holds. Each section that survives is included whole; we don't slice
//! mid-section.

use super::types::{ContextKind, DiscoveredFile, LoadedSkill};

/// Total byte cap for the augmented system prompt.
pub const MAX_TOTAL_BYTES: usize = 50 * 1024;

/// Build the augmented system prompt.
pub fn build_system_prompt(
    persona: &str,
    ctx: &[DiscoveredFile],
    skill: Option<&LoadedSkill>,
) -> String {
    let claude = nearest_of(ctx, ContextKind::ClaudeMd);
    let agents = nearest_distinct_agents(ctx, claude);
    let sections = render_sections(persona, claude, agents, skill);
    fit_to_cap(sections)
}

/// First entry matching `kind` (nearest-first invariant from `discover`).
fn nearest_of(ctx: &[DiscoveredFile], kind: ContextKind) -> Option<&DiscoveredFile> {
    ctx.iter().find(|f| f.kind == kind)
}

/// Nearest AGENTS.md whose path differs from the picked CLAUDE.md (so a
/// symlink CLAUDE.md → AGENTS.md doesn't double-inject).
fn nearest_distinct_agents<'a>(
    ctx: &'a [DiscoveredFile],
    claude: Option<&DiscoveredFile>,
) -> Option<&'a DiscoveredFile> {
    ctx.iter()
        .find(|f| f.kind == ContextKind::AgentsMd && Some(&f.path) != claude.map(|c| &c.path))
}

/// Render each named section as a `(label, body)` pair in injection order.
/// `persona` is index 0 and is never droppable.
fn render_sections(
    persona: &str,
    claude: Option<&DiscoveredFile>,
    agents: Option<&DiscoveredFile>,
    skill: Option<&LoadedSkill>,
) -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = Vec::new();
    v.push(("Persona".to_string(), persona.to_string()));
    if let Some(c) = claude {
        v.push((format!("Project context: {}", c.path.display()), c.content.clone()));
    }
    if let Some(a) = agents {
        v.push((format!("Cross-tool context: {}", a.path.display()), a.content.clone()));
    }
    if let Some(s) = skill {
        v.push((format!("Active skill /{}: {}", s.name, s.path.display()), s.body.clone()));
    }
    v
}

/// Drop trailing sections (skill → agents → claude) until total ≤ cap.
/// Persona at index 0 is always kept.
fn fit_to_cap(mut sections: Vec<(String, String)>) -> String {
    while total_len(&sections) > MAX_TOTAL_BYTES && sections.len() > 1 {
        sections.pop();
    }
    let s = render(&sections);
    if s.len() > MAX_TOTAL_BYTES {
        truncate_persona_only(s)
    } else {
        s
    }
}

/// Sum of formatted section lengths (cheap upper-bound estimate).
fn total_len(sections: &[(String, String)]) -> usize {
    sections
        .iter()
        .map(|(label, body)| label.len() + body.len() + 16)
        .sum()
}

/// Stitch sections into a single prompt with `\n\n=== <label> ===\n` headers.
fn render(sections: &[(String, String)]) -> String {
    let mut out = String::new();
    for (i, (label, body)) in sections.iter().enumerate() {
        if i == 0 {
            out.push_str(body);
        } else {
            out.push_str("\n\n=== ");
            out.push_str(label);
            out.push_str(" ===\n");
            out.push_str(body);
        }
    }
    out
}

/// Last-resort cap when persona alone exceeds the limit. Truncates with
/// a marker rather than panicking.
fn truncate_persona_only(s: String) -> String {
    let mut cut = MAX_TOTAL_BYTES;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    let mut out = s[..cut].to_owned();
    out.push_str("\n[truncated]");
    out
}
