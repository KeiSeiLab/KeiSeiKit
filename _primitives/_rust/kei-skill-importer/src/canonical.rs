//! Canonical AST for imported skills.
//!
//! `ImportedSkill` is the lingua franca between parsers and emitters.
//! Each parser maps its source-specific shape into this struct; each
//! emitter consumes ONLY this struct (never the source-specific raw).
//!
//! Privacy note: the raw `yaml_frontmatter` value is exposed publicly so
//! that emitters can preserve fidelity (round-trip), but it is NOT meant
//! to be inspected by downstream code in lieu of the parsed fields.

use serde::Serialize;
use serde_yaml_ng::Value as YamlValue;
use std::path::PathBuf;

/// Source format of a skill file. `Auto` triggers detection by extension
/// + content sniffing in `parsers::detect_format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SourceFormat {
    OpenClaw,
    Cline,
    Cursor,
    ClaudeCode,
    Kimi,
    Auto,
}

impl SourceFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceFormat::OpenClaw => "openclaw",
            SourceFormat::Cline => "cline",
            SourceFormat::Cursor => "cursor",
            SourceFormat::ClaudeCode => "claude",
            SourceFormat::Kimi => "kimi",
            SourceFormat::Auto => "auto",
        }
    }
}

/// Top-level canonical representation of an imported skill.
///
/// The `body` field holds the markdown body **after** frontmatter has
/// been stripped. `phases` is populated from H2-H3 sections by parsers
/// that recognise the multi-phase wizard pattern; for flat skills it
/// contains a single phase mirroring `body`.
#[derive(Debug, Clone, Serialize)]
pub struct ImportedSkill {
    pub name: String,
    pub description: String,
    pub source_format: SourceFormat,
    pub source_path: PathBuf,
    pub language: Option<String>,
    pub tags: Vec<String>,
    pub phases: Vec<Phase>,
    pub tools_required: Vec<String>,
    #[serde(skip)]
    pub yaml_frontmatter: Option<YamlValue>,
    pub body: String,
}

/// A logical phase / section / step inside a skill. For flat skills
/// (Cursor `.mdc`, Cline single-rule), the parser emits a single
/// `Phase { name: skill.name, body: skill.body, atom_calls: ... }`.
#[derive(Debug, Clone, Serialize)]
pub struct Phase {
    pub name: String,
    pub body: String,
    pub atom_calls: Vec<AtomCall>,
}

/// An invocation detected inside a phase body. `atom_id` is `Some`
/// only when the call resolves against the known KeiSeiKit registry
/// (`kei-cortex::chat`, `kei-task::create`, …) — otherwise `None` and
/// the emitter routes the skill to `as_primitive` (proposal).
#[derive(Debug, Clone, Serialize)]
pub struct AtomCall {
    pub raw_command: String,
    pub atom_id: Option<String>,
    pub kind: AtomCallKind,
}

/// Coarse classification of a detected call site. `Bash` is a generic
/// shell invocation (no `kei-*` prefix); `KeiPrimitive` is a recognised
/// `kei-<crate> <verb>` shape; `UserPrompt` is a slash-command (`/foo`);
/// `Unknown` is everything else (rare).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AtomCallKind {
    Bash,
    KeiPrimitive,
    UserPrompt,
    Unknown,
}

impl AtomCallKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            AtomCallKind::Bash => "bash",
            AtomCallKind::KeiPrimitive => "kei-primitive",
            AtomCallKind::UserPrompt => "user-prompt",
            AtomCallKind::Unknown => "unknown",
        }
    }
}

impl ImportedSkill {
    /// Total number of detected atom calls across all phases.
    pub fn total_atom_calls(&self) -> usize {
        self.phases.iter().map(|p| p.atom_calls.len()).sum()
    }

    /// Number of atom calls that resolved to a known atom_id.
    pub fn resolved_atom_calls(&self) -> usize {
        self.phases
            .iter()
            .flat_map(|p| p.atom_calls.iter())
            .filter(|c| c.atom_id.is_some())
            .count()
    }

    /// Effective body byte length (max of top-level body or sum of
    /// phase bodies — avoids double-counting when phases were split
    /// FROM the same body).
    pub fn body_bytes(&self) -> usize {
        let phases_total: usize = self.phases.iter().map(|p| p.body.len()).sum();
        phases_total.max(self.body.len())
    }
}
