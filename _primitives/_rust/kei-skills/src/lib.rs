//! kei-skills — Hermes / agentskills.io SKILL.md format primitives.
//!
//! Constructor Pattern: one cube per concern.
//! - `format`     — parse/serialize SKILL.md (YAML frontmatter + markdown body)
//! - `validator`  — port of Hermes `_validate_frontmatter` + size caps
//! - `patcher`    — fuzzy find-replace via `similar` crate (atomic write)
//! - `loader`     — walk a directory and load every valid SKILL.md
//! - `registry`   — name-keyed in-memory store with optional hot-reload
//!
//! Bidirectional Hermes interop: same on-disk format, same `extra_taps`
//! distribution. Reading a Hermes skill round-trips byte-equal through
//! `format::parse → format::serialize`.

pub mod format;
pub mod loader;
pub mod patcher;
pub mod registry;
pub mod validator;

pub use format::{Skill, SkillFrontmatter};
pub use loader::{load_all, LoadOutcome};
pub use patcher::{patch_skill, PatchError};
pub use registry::SkillRegistry;
pub use validator::{validate, ValidationIssue, MAX_SKILL_CONTENT_CHARS, MAX_SKILL_FILE_BYTES};
