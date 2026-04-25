//! Wikilink parsing and classification.
//!
//! Covers the strict `[[target]]` matcher used by `kei-sage` and
//! `kei-runtime` to link atom docs to each other and to rule files.

/// Parse a single wikilink `[[target]]`. Returns `None` if not a wikilink,
/// empty, or if the inner body contains a stray bracket (e.g. `[[[foo]]`).
pub fn parse_wikilink(raw: &str) -> Option<String> {
    let t = raw.trim();
    let inner = t.strip_prefix("[[").and_then(|s| s.strip_suffix("]]"))?;
    let inner = inner.trim();
    if inner.is_empty() || inner.contains('[') || inner.contains(']') {
        None
    } else {
        Some(inner.to_string())
    }
}

/// Heuristic atom-target filter: `<crate>::<verb>` looks like an atom,
/// everything starting with `rules/` or `rule ` is a rule reference.
pub fn is_atom_target(target: &str) -> bool {
    !target.starts_with("rules/") && !target.starts_with("rule ")
}

/// Classified wikilink target — atom, rule reference, or other (notes etc.).
///
/// `Rule(slug)` strips the `rules/` prefix and drops any optional `RULE `
/// token, leaving a caller-friendly slug (`"0.12"`, `"memory-protocol"`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WikilinkTarget {
    Atom(String),
    Rule(String),
    Other(String),
}

/// Classify a wikilink inner body. `inner` is the already-unwrapped target
/// (no `[[ ]]`). Use this on the output of `parse_wikilink`.
pub fn classify_wikilink(inner: &str) -> WikilinkTarget {
    let t = inner.trim();
    if let Some(rest) = t.strip_prefix("rules/") {
        return WikilinkTarget::Rule(normalize_rule_slug(rest));
    }
    if let Some(rest) = t.strip_prefix("rule ") {
        return WikilinkTarget::Rule(normalize_rule_slug(rest));
    }
    if is_atom_target(t) && t.contains("::") {
        WikilinkTarget::Atom(t.to_string())
    } else {
        WikilinkTarget::Other(t.to_string())
    }
}

/// Normalise the tail after `rules/` or `rule ` into a short slug.
/// `"RULE 0.12"` → `"0.12"`, `"memory-protocol"` → `"memory-protocol"`.
fn normalize_rule_slug(rest: &str) -> String {
    let r = rest.trim();
    if let Some(tail) = r.strip_prefix("RULE ") {
        return tail.trim().to_string();
    }
    r.to_string()
}
