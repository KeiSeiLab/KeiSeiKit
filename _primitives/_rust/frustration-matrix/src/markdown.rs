//! Markdown chatlog parser — extract USER blocks only.
//!
//! Four block conventions coexist in our chatlog archive:
//!   1. `### User ...` or `## User ...`     (header-delimited, most common)
//!   2. `**User:**`                          (inline-bold label)
//!   3. `> user: ...` quote-style            (quote block label)
//!   4. raw assistant/user lines in `<User>` / `<Assistant>` XML-ish tags
//!
//! Block scope: starts at a USER marker, ends at the next ASSISTANT marker
//! OR the next same-or-higher header. Everything in scope = USER text.
//!
//! We emit ONE `UserLine` per non-empty, non-marker line in a user block —
//! this way `line_no` in output rows points at an actual line with content,
//! which is what a reviewer wants when they open the file in an editor.

use std::path::Path;

/// One user-written line extracted from a chatlog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserLine {
    pub file: String,
    pub line_no: usize,
    pub text: String,
}

/// Role detected for a single line of markdown.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Role {
    User,
    Assistant,
    Other,
}

/// Parse a whole markdown buffer; file path is stamped into each line.
pub fn parse(path: &Path, body: &str) -> Vec<UserLine> {
    let file = path.display().to_string();
    let mut in_user = false;
    let mut out = Vec::new();
    for (idx, raw) in body.lines().enumerate() {
        match classify(raw) {
            Role::User => {
                in_user = true;
                continue;
            }
            Role::Assistant => {
                in_user = false;
                continue;
            }
            Role::Other => {}
        }
        if in_user {
            if let Some(text) = pluck_text(raw) {
                out.push(UserLine {
                    file: file.clone(),
                    line_no: idx + 1,
                    text,
                });
            }
        }
    }
    out
}

/// Classify a line's role marker. We only look at the first ~64 chars —
/// long lines with incidental "User" word in the middle are not markers.
fn classify(line: &str) -> Role {
    // char-boundary-safe head: take up to 96 BYTES but truncate at the last
    // valid UTF-8 boundary ≤ 96. Matters for Cyrillic and symbols like `×`
    // which are multi-byte.
    let cap = line.len().min(96);
    let mut cut = cap;
    while cut > 0 && !line.is_char_boundary(cut) {
        cut -= 1;
    }
    let head = &line[..cut];
    if is_user_marker(head) {
        Role::User
    } else if is_assistant_marker(head) {
        Role::Assistant
    } else {
        Role::Other
    }
}

fn is_user_marker(head: &str) -> bool {
    let t = head.trim_start();
    t.starts_with("### User")
        || t.starts_with("## User")
        || t.starts_with("# User")
        || t.starts_with("**User:**")
        || t.starts_with("**User**:")
        || t.starts_with("> user:")
        || t.starts_with("> User:")
        || t.starts_with("<User>")
        || t.starts_with("<user>")
        || eq_role_header(t, "user")
}

fn is_assistant_marker(head: &str) -> bool {
    let t = head.trim_start();
    t.starts_with("### Assistant")
        || t.starts_with("## Assistant")
        || t.starts_with("# Assistant")
        || t.starts_with("**Assistant:**")
        || t.starts_with("**Assistant**:")
        || t.starts_with("> assistant:")
        || t.starts_with("> Assistant:")
        || t.starts_with("</User>")
        || t.starts_with("<Assistant>")
        || t.starts_with("<assistant>")
        || eq_role_header(t, "assistant")
}

/// Match `role:` bare lines like `user:` or `assistant:` (JSONL-rendered).
fn eq_role_header(t: &str, role: &str) -> bool {
    let lower = t.to_ascii_lowercase();
    lower.starts_with(&format!("{role}:"))
}

/// Strip quote / list prefixes; return Some only if non-empty text remains.
fn pluck_text(line: &str) -> Option<String> {
    let t = line.trim();
    if t.is_empty() {
        return None;
    }
    if t.starts_with("```") || t.starts_with("---") {
        return None;
    }
    let stripped = t
        .trim_start_matches('>')
        .trim_start_matches(' ')
        .trim_start_matches('-')
        .trim_start_matches('*')
        .trim_start_matches(' ')
        .to_string();
    if stripped.is_empty() {
        None
    } else {
        Some(stripped)
    }
}
