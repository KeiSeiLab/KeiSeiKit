//! Filesystem and string helpers for `plan.rs` — atomic write + TOML
//! quoting + inline-table render. Split out to keep `plan.rs` under the
//! 200-LOC ceiling (RULE ZERO).

use anyhow::{anyhow, Result};
use std::path::Path;

pub fn combine(original: &str, block: &str) -> String {
    let mut s = original.to_string();
    if !s.ends_with('\n') {
        s.push('\n');
    }
    s.push_str(block);
    s
}

pub fn quote(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

/// Convert serialized evidence (multi-line `key = value`) into a single
/// inline-table line: `evidence = { kind = "x", file = "y" }`.
pub fn inline_evidence(serialized: &str) -> String {
    let parts: Vec<String> = serialized
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .collect();
    format!("evidence = {{ {} }}", parts.join(", "))
}

pub fn md_escape(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

pub fn atomic_write(target: &Path, contents: &str) -> Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| anyhow!("target {} has no parent", target.display()))?;
    let fname = target
        .file_name()
        .ok_or_else(|| anyhow!("target {} has no file name", target.display()))?;
    let mut tmp_name = std::ffi::OsString::from(".");
    tmp_name.push(fname);
    tmp_name.push(".tmp");
    let tmp = parent.join(tmp_name);
    std::fs::write(&tmp, contents)
        .map_err(|e| anyhow!("write tmp {}: {}", tmp.display(), e))?;
    std::fs::rename(&tmp, target)
        .map_err(|e| anyhow!("rename {} -> {}: {}", tmp.display(), target.display(), e))?;
    Ok(())
}
