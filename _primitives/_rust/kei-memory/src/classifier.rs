//! Event-class classifier — replaces ingest::classify_default.
//!
//! Constructor Pattern: this cube only emits a class label.
//! Persistence + extraction live elsewhere. Order-of-precedence is
//! intentional and documented in `classify` — most specific first.
//!
//! Wave A motive — old `classify_default` had three hardcoded substring
//! checks (permission_denied / worktree_error / cargo_workspace) and no
//! explicit table. Hard to extend, hard to test, no recurrence-class
//! support for "user_correction" / "retry_loop" patterns the audit
//! self-loop relies on.

use regex::Regex;
use std::sync::OnceLock;

/// Pre-compiled regex set. Lazy-initialised on first `classify` call.
///
/// All regex patterns below are compile-time constants validated by the
/// crate's own unit tests; `Regex::new(...).unwrap()` is therefore safe.
/// Same pattern is already used in `injection_patterns.rs::rx`. If the
/// pattern is malformed the failure is caught the first time `classify`
/// runs in tests (panic is the desired sentinel — there is no recovery
/// path for a bad library-author regex).
fn permission_denied_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)permission\s+denied|access\s+denied").unwrap())
}

fn user_correction_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        // English + Russian "you-broke-something" cues. Used to detect
        // recurring user corrections inside one session.
        Regex::new(
            r"(?i)\b(again|stop\s+doing|don'?t\s+(do|repeat)|you'?re\s+wrong|broken|wrong\s+(again|once\s+more))\b|опять|ошибся|не\s+делай",
        )
        .unwrap()
    })
}

fn retry_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)retry|retrying|attempt\s+\d+|try\s+again").unwrap())
}

fn worktree_error_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)worktree.*(error|denied|fail)").unwrap())
}

fn cargo_workspace_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)cargo.*workspace|workspace.*cargo").unwrap())
}

/// Classify one event into a stable label.
///
/// Order of precedence (most specific first):
///   1. tool_error (when is_error and tool present)
///   2. message-level patterns: permission_denied, user_correction,
///      worktree_error, cargo_workspace, retry_loop
///   3. structural fallback: tool_use:<name> for assistant lines with tool,
///      tool_result for user lines with tool, kind for any other typed
///      line, else "other".
pub fn classify(
    kind: Option<&str>,
    tool: Option<&str>,
    message: Option<&str>,
    is_error: bool,
) -> String {
    if let Some(label) = classify_error(tool, is_error) {
        return label;
    }
    if let Some(label) = classify_message(message) {
        return label;
    }
    classify_structural(kind, tool)
}

fn classify_error(tool: Option<&str>, is_error: bool) -> Option<String> {
    if !is_error {
        return None;
    }
    Some(match tool {
        Some(t) => format!("tool_error:{t}"),
        None => "tool_error".to_string(),
    })
}

fn classify_message(message: Option<&str>) -> Option<String> {
    let m = message?;
    if permission_denied_re().is_match(m) {
        return Some("permission_denied".into());
    }
    if user_correction_re().is_match(m) {
        return Some("user_correction".into());
    }
    if worktree_error_re().is_match(m) {
        return Some("worktree_error".into());
    }
    if cargo_workspace_re().is_match(m) {
        return Some("cargo_workspace".into());
    }
    if retry_re().is_match(m) {
        return Some("retry_loop".into());
    }
    None
}

fn classify_structural(kind: Option<&str>, tool: Option<&str>) -> String {
    match (kind, tool) {
        (Some("assistant"), Some(t)) => format!("tool_use:{t}"),
        (Some("user"), Some(_)) => "tool_result".to_string(),
        // Back-compat with old flat traces still using kind="tool_use":
        (Some("tool_use"), Some(t)) => format!("tool_use:{t}"),
        (Some(k), _) => k.to_string(),
        _ => "other".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_error_takes_precedence() {
        let c = classify(Some("user"), Some("Bash"), Some("worktree fail"), true);
        assert_eq!(c, "tool_error:Bash");
    }

    #[test]
    fn permission_denied_matched() {
        let c = classify(Some("user"), None, Some("Permission denied"), false);
        assert_eq!(c, "permission_denied");
    }

    #[test]
    fn user_correction_english() {
        let c = classify(Some("user"), None, Some("you did this again"), false);
        assert_eq!(c, "user_correction");
    }

    #[test]
    fn user_correction_russian() {
        let c = classify(Some("user"), None, Some("опять не работает"), false);
        assert_eq!(c, "user_correction");
    }

    #[test]
    fn assistant_with_tool_emits_tool_use_class() {
        let c = classify(Some("assistant"), Some("Read"), None, false);
        assert_eq!(c, "tool_use:Read");
    }

    #[test]
    fn user_with_tool_emits_tool_result_class() {
        let c = classify(Some("user"), Some("Read"), None, false);
        assert_eq!(c, "tool_result");
    }

    #[test]
    fn legacy_kind_tool_use_still_classifies() {
        let c = classify(Some("tool_use"), Some("Bash"), None, false);
        assert_eq!(c, "tool_use:Bash");
    }

    #[test]
    fn unknown_kind_falls_through_to_other() {
        let c = classify(None, None, None, false);
        assert_eq!(c, "other");
    }
}
