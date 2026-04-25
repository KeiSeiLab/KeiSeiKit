//! Substrate-role expansion — reads `_roles/<name>.toml` and pulls each
//! capability's `text.md` for injection into the generated agent prompt.
//!
//! Constructor Pattern: one cube = one concern. This module does ONLY
//! role → capability-fragments, nothing else. `assembler.rs` calls into
//! it when a manifest declares `substrate_role`.

use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
struct RoleFile {
    #[serde(default)]
    capabilities: RoleCapabilities,
}

#[derive(Default, Deserialize)]
struct RoleCapabilities {
    #[serde(default)]
    required: Vec<String>,
}

/// Load `_roles/<role>.toml` and return the ordered capability names
/// listed under `[capabilities] required`.
pub fn load_role_capabilities(root: &Path, role: &str) -> Result<Vec<String>, String> {
    let path = root.join("_roles").join(format!("{role}.toml"));
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("read role {}: {e}", path.display()))?;
    let parsed: RoleFile = toml::from_str(&text)
        .map_err(|e| format!("parse role {}: {e}", path.display()))?;
    if parsed.capabilities.required.is_empty() {
        return Err(format!(
            "role '{role}' at {} has no [capabilities] required list",
            path.display()
        ));
    }
    Ok(parsed.capabilities.required)
}

/// Load a capability's `text.md` fragment.
///
/// `cap_name` is `<category>::<slug>` (e.g. `policy::no-git-ops`).
pub fn load_capability_text(root: &Path, cap_name: &str) -> Result<String, String> {
    let (category, slug) = split_cap_name(cap_name)?;
    let path = root
        .join("_capabilities")
        .join(category)
        .join(slug)
        .join("text.md");
    std::fs::read_to_string(&path)
        .map_err(|e| format!("read capability {cap_name} at {}: {e}", path.display()))
}

fn split_cap_name(cap: &str) -> Result<(&str, &str), String> {
    match cap.split_once("::") {
        Some((cat, slug)) if !cat.is_empty() && !slug.is_empty() => Ok((cat, slug)),
        _ => Err(format!(
            "malformed capability name '{cap}' — expected <cat>::<slug>"
        )),
    }
}

/// Build the full substrate block: `# AGENT SUBSTRATE` header + each
/// fragment joined with the canonical `\n\n---\n\n` separator used by
/// `kei-agent-runtime::compose`.
pub fn build_substrate_section(root: &Path, role: &str) -> Result<String, String> {
    let caps = load_role_capabilities(root, role)?;
    let mut fragments: Vec<String> = Vec::with_capacity(caps.len());
    for cap in &caps {
        let text = load_capability_text(root, cap)?;
        fragments.push(text.trim().to_string());
    }
    let mut out = String::new();
    out.push_str("# AGENT SUBSTRATE — role `");
    out.push_str(role);
    out.push_str("`\n\n");
    out.push_str("> Enforced by `kei-capability` gates + verifies. The rules below are not advisory.\n\n");
    out.push_str(&fragments.join("\n\n---\n\n"));
    out.push_str("\n\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_cap_name_ok() {
        assert_eq!(split_cap_name("policy::no-git-ops").unwrap(), ("policy", "no-git-ops"));
    }

    #[test]
    fn split_cap_name_rejects_missing_sep() {
        assert!(split_cap_name("policy-no-git-ops").is_err());
    }

    #[test]
    fn split_cap_name_rejects_empty_side() {
        assert!(split_cap_name("::slug").is_err());
        assert!(split_cap_name("cat::").is_err());
    }
}
