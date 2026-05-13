//! Agent-shell DNA — 5-segment per-invocation identifier.
//!
//! Format emitted by `keisei-marketplace/src/lib/cryptoid.ts::agentDna`:
//!
//!   `agent-shell::<provider>:<model>:<caps>::<scope_sha>::<body_sha>-<nonce>`
//!
//! Where:
//!   - provider, model — kebab-case slug, 1..=64 chars `[a-z0-9_.-]`
//!   - caps            — capability bundle code, 1..=32 chars `[A-Z0-9-]`
//!   - scope_sha       — lower-case hex, 8 OR 16 chars (legacy 8, new 16)
//!   - body_sha        — same shape as scope_sha
//!   - nonce           — lower-case hex, 8 OR 16 chars (legacy 8, new 16)
//!
//! This cube is purely lexical: no I/O, no SQL, no panics on input.
//! Companion to `dna_class.rs` (legacy 4-segment format).

/// Parsed agent-shell DNA. All fields hold borrowed slices into the input.
#[derive(Debug, PartialEq, Eq)]
pub struct AgentShellDna<'a> {
    pub provider: &'a str,
    pub model: &'a str,
    pub caps: &'a str,
    pub scope_sha: &'a str,
    pub body_sha: &'a str,
    pub nonce: &'a str,
}

const PREFIX: &str = "agent-shell::";

/// Parse a marketplace-emitted agent-shell DNA. Accepts both legacy
/// (8-hex scope/body/nonce) and current (16-hex scope/body, 16-hex nonce)
/// length conventions. Returns None on any malformed input.
pub fn parse(dna: &str) -> Option<AgentShellDna<'_>> {
    let rest = dna.strip_prefix(PREFIX)?;
    let mut segs = rest.splitn(4, "::");
    let triple = segs.next()?;
    let scope_sha = segs.next()?;
    let body_and_nonce = segs.next()?;
    if segs.next().is_some() {
        return None;
    }

    let mut triple_parts = triple.split(':');
    let provider = triple_parts.next()?;
    let model = triple_parts.next()?;
    let caps = triple_parts.next()?;
    if triple_parts.next().is_some() {
        return None;
    }
    if !is_slug(provider) || !is_slug(model) || !is_caps(caps) {
        return None;
    }

    if !is_hex_len(scope_sha, &[8, 16]) {
        return None;
    }

    let dash = body_and_nonce.find('-')?;
    let body_sha = &body_and_nonce[..dash];
    let nonce = &body_and_nonce[dash + 1..];
    if !is_hex_len(body_sha, &[8, 16]) {
        return None;
    }
    if !is_hex_len(nonce, &[8, 16]) {
        return None;
    }

    Some(AgentShellDna {
        provider,
        model,
        caps,
        scope_sha,
        body_sha,
        nonce,
    })
}

/// Drop trailing `-<nonce>` to obtain the task-class identifier.
/// Same prompt re-runs cluster on the same task-class.
pub fn task_class<'a>(dna: &'a str) -> Option<&'a str> {
    let _ = parse(dna)?;
    let dash = dna.rfind('-')?;
    Some(&dna[..dash])
}

/// Drop `::<body_sha>-<nonce>` to obtain the agent-class identifier:
/// `agent-shell::<provider>:<model>:<caps>::<scope_sha>`.
pub fn agent_class<'a>(dna: &'a str) -> Option<&'a str> {
    let task = task_class(dna)?;
    let last_sep = task.rfind("::")?;
    Some(&task[..last_sep])
}

fn is_slug(s: &str) -> bool {
    if s.is_empty() || s.len() > 64 {
        return false;
    }
    let bytes = s.as_bytes();
    if !is_slug_head(bytes[0]) {
        return false;
    }
    bytes[1..].iter().all(|&b| is_slug_tail(b))
}

fn is_slug_head(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'0'..=b'9')
}

fn is_slug_tail(b: u8) -> bool {
    matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b'-')
}

fn is_caps(s: &str) -> bool {
    if s.is_empty() || s.len() > 32 {
        return false;
    }
    let bytes = s.as_bytes();
    if !matches!(bytes[0], b'A'..=b'Z') {
        return false;
    }
    bytes[1..]
        .iter()
        .all(|&b| matches!(b, b'A'..=b'Z' | b'0'..=b'9' | b'-'))
}

fn is_hex_len(s: &str, allowed: &[usize]) -> bool {
    if !allowed.contains(&s.len()) {
        return false;
    }
    s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

#[cfg(test)]
mod tests {
    use super::*;

    const NEW: &str = "agent-shell::anthropic:claude-sonnet-4-6:FS-RW-BASH-PLAN::a903a13f18b7336c::fabd290e1234abcd-deadbeef12345678";
    const LEGACY: &str = "agent-shell::openai:gpt-5-codex:FS-RO::abcdef12::34567890-aabbccdd";

    #[test]
    fn parses_new_format_16hex() {
        let p = parse(NEW).expect("parse new");
        assert_eq!(p.provider, "anthropic");
        assert_eq!(p.model, "claude-sonnet-4-6");
        assert_eq!(p.caps, "FS-RW-BASH-PLAN");
        assert_eq!(p.scope_sha, "a903a13f18b7336c");
        assert_eq!(p.body_sha, "fabd290e1234abcd");
        assert_eq!(p.nonce, "deadbeef12345678");
    }

    #[test]
    fn parses_legacy_8hex() {
        let p = parse(LEGACY).expect("parse legacy");
        assert_eq!(p.provider, "openai");
        assert_eq!(p.model, "gpt-5-codex");
        assert_eq!(p.caps, "FS-RO");
        assert_eq!(p.scope_sha, "abcdef12");
        assert_eq!(p.body_sha, "34567890");
        assert_eq!(p.nonce, "aabbccdd");
    }

    #[test]
    fn rejects_missing_prefix() {
        assert!(parse("openai:gpt-5:FS-RO::deadbeef::cafebabe-1234abcd").is_none());
    }

    #[test]
    fn rejects_uppercase_provider() {
        let bad = "agent-shell::Anthropic:claude-sonnet-4-6:FS-RW::abcdef12::34567890-aabbccdd";
        assert!(parse(bad).is_none());
    }

    #[test]
    fn rejects_lowercase_caps() {
        let bad = "agent-shell::anthropic:claude-sonnet-4-6:fs-rw::abcdef12::34567890-aabbccdd";
        assert!(parse(bad).is_none());
    }

    #[test]
    fn rejects_wrong_hex_length() {
        let bad = "agent-shell::anthropic:claude:FS-RW::abcdef1::34567890-aabbccdd"; // 7-hex scope
        assert!(parse(bad).is_none());
    }

    #[test]
    fn rejects_non_hex_chars() {
        let bad = "agent-shell::anthropic:claude:FS-RW::abcdefgh::34567890-aabbccdd"; // 'g','h' not hex
        assert!(parse(bad).is_none());
    }

    #[test]
    fn rejects_extra_triple_field() {
        let bad = "agent-shell::a:b:C:D::abcdef12::34567890-aabbccdd";
        assert!(parse(bad).is_none());
    }

    #[test]
    fn rejects_empty_input() {
        assert!(parse("").is_none());
    }

    #[test]
    fn rejects_missing_dash_in_nonce_pair() {
        let bad = "agent-shell::anthropic:claude:FS-RW::abcdef12::34567890aabbccdd";
        assert!(parse(bad).is_none());
    }

    #[test]
    fn task_class_strips_nonce() {
        assert_eq!(
            task_class(LEGACY),
            Some("agent-shell::openai:gpt-5-codex:FS-RO::abcdef12::34567890")
        );
    }

    #[test]
    fn agent_class_strips_body_and_nonce() {
        assert_eq!(
            agent_class(LEGACY),
            Some("agent-shell::openai:gpt-5-codex:FS-RO::abcdef12")
        );
    }

    #[test]
    fn task_and_agent_class_reject_malformed() {
        assert_eq!(task_class("not-an-agent-shell"), None);
        assert_eq!(agent_class("agent-shell::a:b::no-good"), None);
    }
}
