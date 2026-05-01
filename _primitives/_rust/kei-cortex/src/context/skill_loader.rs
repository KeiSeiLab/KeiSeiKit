//! Match a leading `/skill-name` token in the user's first turn and load
//! the corresponding `SKILL.md` body.
//!
//! Resolution order (project-local wins):
//!   1. `<project_root>/.claude/skills/<name>/SKILL.md`
//!   2. `~/.claude/skills/<name>/SKILL.md`
//!
//! Only the FIRST whitespace-delimited token is inspected. If that token
//! does not start with `/`, or the resolved file does not exist, the
//! function returns `None` — the caller treats this as "no skill matched"
//! and falls back to the persona/context-only system prompt.

use super::types::LoadedSkill;
use std::path::{Path, PathBuf};

/// Hard read cap mirroring `discover::MAX_FILE_BYTES`. Skills are usually
/// small (a few KiB), so anything past 1 MiB is almost certainly wrong.
const MAX_SKILL_BYTES: usize = 1024 * 1024;

/// If `user_msg` starts with `/skill-name`, locate and read the
/// corresponding `SKILL.md`. See module docs for resolution order.
pub fn match_skill_command(user_msg: &str, project_root: &Path) -> Option<LoadedSkill> {
    let name = extract_name(user_msg)?;
    if !is_valid_name(name) {
        return None;
    }
    if let Some(s) = try_load(project_root, name) {
        return Some(s);
    }
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    try_load(&home, name)
}

/// Pull the `<name>` out of `/<name>...`. Returns `None` if the message
/// doesn't begin with `/` or the slash is followed by whitespace/EOF.
fn extract_name(msg: &str) -> Option<&str> {
    let trimmed = msg.trim_start();
    let rest = trimmed.strip_prefix('/')?;
    let end = rest
        .find(|c: char| c.is_whitespace())
        .unwrap_or(rest.len());
    if end == 0 {
        return None;
    }
    Some(&rest[..end])
}

/// Reject pathological skill names: empty, path-segment, dot-relative, or
/// non-`[A-Za-z0-9_-]`. Defends against `/../etc/passwd` style escapes.
fn is_valid_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Attempt `<root>/.claude/skills/<name>/SKILL.md` and read it.
fn try_load(root: &Path, name: &str) -> Option<LoadedSkill> {
    let path = root
        .join(".claude")
        .join("skills")
        .join(name)
        .join("SKILL.md");
    let meta = std::fs::symlink_metadata(&path).ok()?;
    if meta.file_type().is_symlink() || !meta.file_type().is_file() {
        return None;
    }
    let raw = std::fs::read_to_string(&path).ok()?;
    let body = if raw.len() > MAX_SKILL_BYTES {
        let mut cut = MAX_SKILL_BYTES;
        while cut > 0 && !raw.is_char_boundary(cut) {
            cut -= 1;
        }
        let mut out = raw[..cut].to_owned();
        out.push_str("\n[truncated]");
        out
    } else {
        raw
    };
    Some(LoadedSkill {
        name: name.to_owned(),
        path,
        body,
    })
}
