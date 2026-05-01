//! Manifest struct — deserialized from _manifests/*.toml.
//! One manifest = one agent. Source of truth; the .md file is generated.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub name: String,
    pub description: String,
    pub tools: Vec<String>,
    pub model: String,
    pub role: String,
    pub blocks: Vec<String>,
    /// v0.16 (phase 5): agent substrate role. When present, assembler loads
    /// `_roles/<substrate_role>.toml` and emits each capability's `text.md`
    /// fragment between the ROLE section and the existing blocks. Optional
    /// for backward compatibility with pre-substrate manifests.
    #[serde(default)]
    pub substrate_role: Option<String>,
    pub domain_in: Vec<String>,
    pub forbidden_domain: Vec<String>,
    pub handoff: Vec<Handoff>,
    #[serde(default)]
    pub output_extra_fields: Vec<String>,
    pub memory_project: Option<String>,
    pub project_claudemd: Option<String>,
    pub references: Option<References>,
    /// v0.15: optional typed-artifact schema this agent emits on completion.
    /// Must be one of the names in `artifact_schemas::KNOWN`.
    #[serde(default)]
    pub produces_artifact: Option<String>,
    /// v0.16 rule_blocks: registry fragment names to inject after blocks.
    /// Format: `"<rule-slug>::<section-slug>"`, e.g.
    /// `"karpathy-behavioral::1-think-before-coding"`.
    /// Fragments are fetched from `~/.claude/registry.sqlite` at assemble time.
    #[serde(default)]
    pub rule_blocks: Vec<String>,
}

#[derive(Deserialize)]
pub struct Handoff {
    pub target: String,
    pub trigger: String,
    /// v0.15: optional schema name the target consumes from this handoff.
    #[serde(default)]
    pub expects_artifact: Option<String>,
    /// v0.15: optional schema name this agent produces for the target.
    #[serde(default)]
    pub produces_artifact: Option<String>,
}

#[derive(Deserialize)]
pub struct References {
    #[serde(default)]
    pub extra: Vec<String>,
}
