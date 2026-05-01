//! kei-skill-importer — parse external AI-coding-tool skill files and
//! emit them in KeiSeiKit canonical shapes.
//!
//! Pipeline: `parse → canonicalize (ImportedSkill) → classify (atom-calls)
//! → decide emit-path → emit (atom / recipe / proposed-primitive)`.
//!
//! Side-effect-free at the library surface: parsers and the classifier
//! never write to disk. Only `emit::*::write` functions persist files,
//! and only when handed an explicit `output_dir`.

pub mod canonical;
pub mod classifier;
pub mod emit;
pub mod parsers;

// Reserved for Wave 27: dynamic atom discovery via the shared crate.
// Importing the crate here documents the forward dependency and keeps
// the workspace lockfile consistent.
#[allow(unused_imports)]
use kei_atom_discovery as _atom_discovery_reserved;

use anyhow::{bail, Context, Result};
use std::path::Path;

pub use canonical::{AtomCall, AtomCallKind, ImportedSkill, Phase, SourceFormat};
pub use emit::{decide_emit_path, EmitPath};

/// Canonical entry point: parse a skill file at `path` using the format
/// hint (`SourceFormat::Auto` triggers detection by extension/content).
///
/// Returns a `ImportedSkill` AST suitable for classification + emission.
pub fn import(path: &Path, format: SourceFormat) -> Result<ImportedSkill> {
    let chosen = match format {
        SourceFormat::Auto => parsers::detect_format(path),
        other => other,
    };
    let mut skill = match chosen {
        SourceFormat::OpenClaw => parsers::openclaw::parse(path),
        SourceFormat::Cline => parsers::cline::parse(path),
        SourceFormat::Cursor => parsers::cursor::parse(path),
        SourceFormat::ClaudeCode => parsers::claude::parse(path),
        SourceFormat::Kimi => parsers::kimi::parse(path),
        SourceFormat::Auto => bail!("format auto-detection failed for {}", path.display()),
    }
    .with_context(|| format!("parse {} (as {:?})", path.display(), chosen))?;
    classifier::classify_atom_calls(&mut skill);
    Ok(skill)
}
