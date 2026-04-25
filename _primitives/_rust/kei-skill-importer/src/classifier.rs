//! Atom-call classifier — detects bash fences, `kei-<crate> <verb>`
//! invocations, and slash commands inside each phase body. Resolves
//! `atom_id` against a static registry (Wave 26.5 scope; Wave 27 will
//! swap this for a live `kei-atom-discovery::discover_atoms` lookup).

use crate::canonical::{AtomCall, AtomCallKind, ImportedSkill};
use regex::Regex;
use std::sync::OnceLock;

/// Static registry of known KeiSeiKit primitive verbs (Wave 26.5).
/// Used by `try_resolve_atom_id`. Wave 27 will swap to dynamic discovery.
const KNOWN_PRIMITIVES: &[&str] = &[
    "kei-cortex", "kei-task", "kei-sage", "kei-router", "kei-cache",
    "kei-pipe", "kei-spawn", "kei-replay", "kei-fork", "kei-ledger",
    "kei-memory", "kei-migrate", "kei-changelog", "kei-pet", "kei-store",
    "kei-artifact", "kei-search-core", "kei-content-store",
    "kei-social-store", "kei-chat-store", "kei-crossdomain",
    "kei-curator", "kei-auth", "kei-entity-store", "kei-agent-runtime",
    "kei-capability", "kei-provision", "kei-discover", "kei-prune",
    "kei-brain-view", "kei-hibernate", "kei-conflict-scan",
    "kei-refactor-engine", "kei-graph-check", "kei-watch",
    "kei-scheduler", "kei-diff", "kei-dna-index", "kei-shared",
    "kei-forge", "kei-runtime", "kei-atom-discovery",
    "kei-skill-importer", "tomd", "ssh-check", "firewall-diff",
    "mock-render", "visual-diff", "tokens-sync",
];

fn bash_fence_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?ms)^```(?:bash|sh|shell)\s*\n(.*?)^```").unwrap())
}

fn slash_cmd_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?m)(?:^|\s)(/[a-z][a-z0-9_-]{1,40})\b").unwrap())
}

fn kei_primitive_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"\b(kei-[a-z][a-z0-9-]+)\s+([a-z][a-z0-9-]{1,30})\b").unwrap())
}

/// Walk every phase, populate `phase.atom_calls`.
pub fn classify_atom_calls(skill: &mut ImportedSkill) {
    for phase in skill.phases.iter_mut() {
        let calls = scan_phase(&phase.body);
        phase.atom_calls = calls;
    }
}

fn scan_phase(body: &str) -> Vec<AtomCall> {
    let mut out: Vec<AtomCall> = Vec::new();
    collect_bash_fences(body, &mut out);
    collect_kei_primitives(body, &mut out);
    collect_slash_commands(body, &mut out);
    dedup(&mut out);
    out
}

fn collect_bash_fences(body: &str, out: &mut Vec<AtomCall>) {
    for cap in bash_fence_re().captures_iter(body) {
        let block = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        for raw in block.lines() {
            let cmd = raw.trim();
            if cmd.is_empty() || cmd.starts_with('#') {
                continue;
            }
            let kind = classify_kind(cmd);
            let atom_id = try_resolve_atom_id(cmd);
            out.push(AtomCall {
                raw_command: cmd.to_string(),
                atom_id,
                kind,
            });
        }
    }
}

fn collect_kei_primitives(body: &str, out: &mut Vec<AtomCall>) {
    for cap in kei_primitive_re().captures_iter(body) {
        let prim = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let verb = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        if !KNOWN_PRIMITIVES.contains(&prim) {
            continue;
        }
        let target_id = format!("{prim}::{verb}");
        // Skip if a bash-fence already captured this call (any raw_command
        // that resolved to the same atom_id).
        if out.iter().any(|c| c.atom_id.as_deref() == Some(target_id.as_str())) {
            continue;
        }
        let raw = format!("{prim} {verb}");
        if out.iter().any(|c| c.raw_command == raw) {
            continue;
        }
        out.push(AtomCall {
            raw_command: raw,
            atom_id: Some(target_id),
            kind: AtomCallKind::KeiPrimitive,
        });
    }
}

fn collect_slash_commands(body: &str, out: &mut Vec<AtomCall>) {
    for cap in slash_cmd_re().captures_iter(body) {
        let cmd = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        // Filter false positives — URL paths, regex anchors.
        if cmd.contains("//") || cmd.len() < 3 {
            continue;
        }
        let raw = cmd.to_string();
        if out.iter().any(|c| c.raw_command == raw) {
            continue;
        }
        out.push(AtomCall {
            raw_command: raw,
            atom_id: None,
            kind: AtomCallKind::UserPrompt,
        });
    }
}

fn classify_kind(cmd: &str) -> AtomCallKind {
    let head = cmd.split_whitespace().next().unwrap_or("");
    if KNOWN_PRIMITIVES.contains(&head) {
        AtomCallKind::KeiPrimitive
    } else if !head.is_empty() {
        AtomCallKind::Bash
    } else {
        AtomCallKind::Unknown
    }
}

fn try_resolve_atom_id(cmd: &str) -> Option<String> {
    let mut parts = cmd.split_whitespace();
    let head = parts.next()?;
    let verb = parts.next()?;
    if !KNOWN_PRIMITIVES.contains(&head) {
        return None;
    }
    if !verb.chars().next()?.is_ascii_lowercase() {
        return None;
    }
    Some(format!("{head}::{verb}"))
}

fn dedup(calls: &mut Vec<AtomCall>) {
    let mut seen = std::collections::HashSet::new();
    calls.retain(|c| seen.insert(c.raw_command.clone()));
}

/// Public predicate used by the emit-path decision.
pub fn has_unresolved_atom_calls(skill: &ImportedSkill) -> bool {
    skill
        .phases
        .iter()
        .flat_map(|p| p.atom_calls.iter())
        .any(|c| c.atom_id.is_none() && c.kind != AtomCallKind::Bash)
}
