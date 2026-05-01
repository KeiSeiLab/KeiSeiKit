//! registry_writer — register identified modules in kei-registry.
//!
//! Each module becomes a `BlockType::Primitive` row. Idempotent: matching
//! body_sha → no-op (unchanged). Differing body_sha → supersede chain.
//!
//! Constructor Pattern: one responsibility (write modules → registry). I/O
//! lives here; DNA composition delegates to kei_registry::register.

use crate::identifier::ProjectModule;
use anyhow::{Context, Result};
use kei_registry::{open_db, register, BlockType};
use std::path::{Path, PathBuf};

/// Summary returned after a register call.
pub struct RegisterResult {
    pub registered: usize,
    pub superseded: usize,
    pub unchanged: usize,
}

/// Register every module in `kei-registry` under `BlockType::Primitive`.
///
/// - `project_root` last path component is the project_slug.
/// - `registry_db` = `None` → resolves via env `KEI_REGISTRY_DB` or
///   `~/.claude/registry.sqlite`.
/// - Idempotent: identical body_sha → no-op. Changed body_sha → supersede.
pub fn register_modules(
    modules: &[ProjectModule],
    project_root: &Path,
    registry_db: Option<&Path>,
) -> Result<RegisterResult> {
    let db_path = resolve_db(registry_db)?;
    let conn = open_db(&db_path)
        .with_context(|| format!("open registry at {}", db_path.display()))?;

    let slug = project_slug(project_root);
    let mut result = RegisterResult { registered: 0, superseded: 0, unchanged: 0 };

    for module in modules {
        let was_new = register_one(&conn, module, project_root, &slug)?;
        match was_new {
            RegistrationOutcome::New => result.registered += 1,
            RegistrationOutcome::Superseded => result.superseded += 1,
            RegistrationOutcome::Unchanged => result.unchanged += 1,
        }
    }
    Ok(result)
}

// ── internals ────────────────────────────────────────────────────────────────

enum RegistrationOutcome {
    New,
    Superseded,
    Unchanged,
}

fn register_one(
    conn: &rusqlite::Connection,
    module: &ProjectModule,
    project_root: &Path,
    slug: &str,
) -> Result<RegistrationOutcome> {
    let name = format!("{}::{}", slug, module.name);
    let abs_manifest = project_root.join(&module.manifest_path);
    let path_str = abs_manifest
        .to_str()
        .with_context(|| format!("non-UTF8 path: {}", abs_manifest.display()))?;

    let body = build_body(module, project_root)?;
    let body_sha = body_sha8(&body);

    // Check if active row exists for this path.
    let existing = kei_registry::find_by_path(conn, path_str)?;
    let outcome = match &existing {
        Some(b) if b.body_sha == body_sha => RegistrationOutcome::Unchanged,
        Some(_) => RegistrationOutcome::Superseded,
        None => RegistrationOutcome::New,
    };

    // register() handles both fresh insert and supersede chain internally.
    register(conn, BlockType::Primitive, &name, path_str, &body, "")
        .with_context(|| format!("register module {name}"))?;

    Ok(outcome)
}

/// Concatenate source file contents in sorted-path order for deterministic SHA.
fn build_body(module: &ProjectModule, project_root: &Path) -> Result<Vec<u8>> {
    let mut paths: Vec<&PathBuf> = module.source_files.iter().collect();
    paths.sort();
    let mut buf = Vec::new();
    for rel in paths {
        let abs = project_root.join(rel);
        match std::fs::read(&abs) {
            Ok(bytes) => buf.extend_from_slice(&bytes),
            Err(_) => {} // missing / unreadable file → skip
        }
    }
    // Fall back to manifest path string so nameless modules still have a body.
    if buf.is_empty() {
        buf.extend_from_slice(module.manifest_path.to_string_lossy().as_bytes());
    }
    Ok(buf)
}

/// 8-hex SHA-256 prefix over raw bytes. Mirrors kei_registry::dna_block usage.
fn body_sha8(body: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let d = Sha256::digest(body);
    format!("{:02x}{:02x}{:02x}{:02x}", d[0], d[1], d[2], d[3])
}

/// Last path component of project root, defaulting to "unknown".
pub fn project_slug(root: &Path) -> String {
    root.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_owned()
}

/// Resolve the registry DB path: explicit → env → default.
fn resolve_db(explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = explicit {
        return Ok(p.to_path_buf());
    }
    if let Ok(v) = std::env::var("KEI_REGISTRY_DB") {
        return Ok(PathBuf::from(v));
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_owned());
    Ok(PathBuf::from(home).join(".claude").join("registry.sqlite"))
}
