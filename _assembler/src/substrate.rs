//! Substrate-role expansion — reads `_roles/<name>.toml` and pulls each
//! capability's `text.md` for injection into the generated agent prompt.
//!
//! Constructor Pattern: one cube = one concern. This module does ONLY
//! role → capability-fragments, nothing else. `assembler.rs` calls into
//! it when a manifest declares `substrate_role`.

use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Deserialize)]
struct RoleFile {
    #[serde(default)]
    capabilities: RoleCapabilities,
}

#[derive(Default, Deserialize)]
struct RoleCapabilities {
    /// Optional parent role — its `required` list is loaded recursively
    /// and combined with this role's `required` (parent first, dedup, then
    /// `relaxes` removed). Cycles are detected and rejected.
    #[serde(default)]
    extends: Option<String>,
    #[serde(default)]
    required: Vec<String>,
    /// Capability names to drop from the parent's `required` list. Only
    /// meaningful when `extends` is set.
    #[serde(default)]
    relaxes: Vec<String>,
}

/// Load `_roles/<role>.toml` and return the ordered capability names.
/// If the role declares `extends`, the parent's required list is loaded
/// recursively and merged (parent first, dedup, `relaxes` applied).
pub fn load_role_capabilities(root: &Path, role: &str) -> Result<Vec<String>, String> {
    let mut visited: HashSet<String> = HashSet::new();
    load_role_capabilities_inner(root, role, &mut visited)
}

fn load_role_capabilities_inner(
    root: &Path,
    role: &str,
    visited: &mut HashSet<String>,
) -> Result<Vec<String>, String> {
    if !visited.insert(role.to_string()) {
        return Err(format!(
            "role '{role}' has a cyclic `extends` chain: {visited:?}"
        ));
    }
    let path = root.join("_roles").join(format!("{role}.toml"));
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("read role {}: {e}", path.display()))?;
    let parsed: RoleFile = toml::from_str(&text)
        .map_err(|e| format!("parse role {}: {e}", path.display()))?;
    let caps = &parsed.capabilities;

    let combined = match &caps.extends {
        Some(parent) => merge_extends(root, parent, &caps.required, &caps.relaxes, visited)?,
        None => caps.required.clone(),
    };

    if combined.is_empty() {
        return Err(format!(
            "role '{role}' at {} resolves to an empty capability list",
            path.display()
        ));
    }
    Ok(combined)
}

/// Resolve `extends` inheritance: load parent's full list, append this
/// role's `required` (skipping duplicates), then remove anything in
/// `relaxes`. Order: parent fragments come first, child overrides come
/// after, child relaxations subtract from the union.
fn merge_extends(
    root: &Path,
    parent_role: &str,
    own_required: &[String],
    relaxes: &[String],
    visited: &mut HashSet<String>,
) -> Result<Vec<String>, String> {
    let parent_caps = load_role_capabilities_inner(root, parent_role, visited)?;
    let mut seen: HashSet<&str> = HashSet::new();
    let mut out: Vec<String> = Vec::with_capacity(parent_caps.len() + own_required.len());
    for c in parent_caps.iter().chain(own_required.iter()) {
        if seen.insert(c.as_str()) {
            out.push(c.clone());
        }
    }
    let drop: HashSet<&str> = relaxes.iter().map(String::as_str).collect();
    out.retain(|c| !drop.contains(c.as_str()));
    Ok(out)
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
    use std::fs;
    use std::path::PathBuf;

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

    fn tmp_kit(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("substrate-test-{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("_roles")).unwrap();
        dir
    }

    fn write_role(root: &Path, name: &str, body: &str) {
        fs::write(root.join("_roles").join(format!("{name}.toml")), body).unwrap();
    }

    #[test]
    fn extends_inherits_parent_required() {
        let root = tmp_kit("inherit");
        write_role(&root, "parent", "[capabilities]\nrequired = [\"a\", \"b\"]\n");
        write_role(&root, "child", "[capabilities]\nextends = \"parent\"\nrequired = [\"c\"]\n");
        let caps = load_role_capabilities(&root, "child").unwrap();
        assert_eq!(caps, vec!["a", "b", "c"]);
    }

    #[test]
    fn extends_with_relaxes_drops_parent_items() {
        let root = tmp_kit("relax");
        write_role(&root, "parent", "[capabilities]\nrequired = [\"a\", \"b\", \"c\"]\n");
        write_role(
            &root,
            "child",
            "[capabilities]\nextends = \"parent\"\nrequired = [\"d\"]\nrelaxes = [\"b\"]\n",
        );
        let caps = load_role_capabilities(&root, "child").unwrap();
        assert_eq!(caps, vec!["a", "c", "d"]);
    }

    #[test]
    fn extends_dedups_when_child_repeats_parent() {
        let root = tmp_kit("dedup");
        write_role(&root, "parent", "[capabilities]\nrequired = [\"a\", \"b\"]\n");
        write_role(
            &root,
            "child",
            "[capabilities]\nextends = \"parent\"\nrequired = [\"b\", \"c\"]\n",
        );
        let caps = load_role_capabilities(&root, "child").unwrap();
        assert_eq!(caps, vec!["a", "b", "c"]);
    }

    #[test]
    fn extends_cycle_rejected() {
        let root = tmp_kit("cycle");
        write_role(&root, "a", "[capabilities]\nextends = \"b\"\nrequired = [\"x\"]\n");
        write_role(&root, "b", "[capabilities]\nextends = \"a\"\nrequired = [\"y\"]\n");
        let err = load_role_capabilities(&root, "a").unwrap_err();
        assert!(err.contains("cyclic"), "err: {err}");
    }

    #[test]
    fn empty_required_no_extends_rejects() {
        let root = tmp_kit("empty");
        write_role(&root, "lonely", "[capabilities]\nrequired = []\n");
        assert!(load_role_capabilities(&root, "lonely").is_err());
    }
}
