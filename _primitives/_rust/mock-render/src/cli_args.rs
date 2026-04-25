//! Shared CLI-arg helpers for every mock-render subcommand.
//!
//! Extracted from `main.rs` in v0.14.1 to keep that dispatcher ≤40 LOC
//! per Constructor Pattern.

use std::path::PathBuf;

/// Look up a `--name <value>` pair in the arg slice.
pub fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == name)
        .map(|w| w[1].as_str())
}

/// Parse `WxH` viewport (e.g. `1280x800`).
pub fn parse_viewport(s: &str) -> Option<(u32, u32)> {
    let (w, h) = s.split_once('x')?;
    Some((w.parse().ok()?, h.parse().ok()?))
}

/// Require `--project` (default `.`) and `--section <existing-file>`.
pub fn require_project_section(args: &[String]) -> Result<(PathBuf, PathBuf), String> {
    let project = flag(args, "--project")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let section = flag(args, "--section")
        .map(PathBuf::from)
        .ok_or_else(|| "--section <file> required".to_string())?;
    if !section.exists() {
        return Err(format!("section file not found: {}", section.display()));
    }
    Ok((project, section))
}
