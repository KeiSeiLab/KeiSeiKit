//! Heuristic classifier — `RawAction.title + severity + effort` → [`ActionKind`].
//!
//! Pure function; no IO. Keyword match on lowercased title decides; severity
//! and effort do NOT change the kind today (reserved for future tuning).

use serde::{Deserialize, Serialize};

use crate::parser::RawAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ActionKind {
    Refactor,
    Migrate,
    NewPrimitive,
    Decompose,
    Doc,
    Unknown,
}

impl ActionKind {
    /// Lower-snake form for use in slugs / file names.
    pub fn slug(&self) -> &'static str {
        match self {
            Self::Refactor => "refactor",
            Self::Migrate => "migrate",
            Self::NewPrimitive => "new-primitive",
            Self::Decompose => "decompose",
            Self::Doc => "doc",
            Self::Unknown => "unknown",
        }
    }
}

/// Pure mapping `RawAction → ActionKind`. Order of checks matters: more
/// specific keywords win.
pub fn classify(action: &RawAction) -> ActionKind {
    let t = action.title.to_lowercase();
    if matches_new_primitive(&t) {
        return ActionKind::NewPrimitive;
    }
    if matches_decompose(&t) {
        return ActionKind::Decompose;
    }
    if matches_migrate(&t) {
        return ActionKind::Migrate;
    }
    if matches_refactor(&t) {
        return ActionKind::Refactor;
    }
    if matches_doc(&t) {
        return ActionKind::Doc;
    }
    ActionKind::Unknown
}

fn matches_new_primitive(t: &str) -> bool {
    contains_any(t, &["new primitive", "build primitive", "implement primitive", "add primitive", "create primitive"])
        || (contains_any(t, &["new", "create", "build", "implement", "add"]) && t.contains("crate"))
}

fn matches_decompose(t: &str) -> bool {
    contains_any(t, &["decompose", "split into", "break up", "extract module"])
}

fn matches_migrate(t: &str) -> bool {
    contains_any(t, &["migrate", "port to", "switch to", "replace with"])
}

fn matches_refactor(t: &str) -> bool {
    contains_any(t, &["refactor", "rewrite", "rework", "restructure", "wrap"])
}

fn matches_doc(t: &str) -> bool {
    contains_any(t, &["document", "docs", "readme", "doc-only", "wiki", "translator"])
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|n| haystack.contains(n))
}
