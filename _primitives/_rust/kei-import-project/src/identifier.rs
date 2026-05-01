//! identifier — find manifest files, parse module names, collect source files.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::walker::{Language, RepoWalk};
use anyhow::Result;
use std::path::PathBuf;

/// Category of a detected project module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleKind {
    /// Cargo.toml (Rust crate or workspace member)
    RustCrate,
    /// package.json (Node/NPM)
    NpmPackage,
    /// pyproject.toml or setup.py (Python)
    PythonPackage,
    /// go.mod (Go module)
    GoModule,
}

/// A language module identified within the walked tree.
pub struct ProjectModule {
    pub kind: ModuleKind,
    /// Root-relative path to the manifest file.
    pub manifest_path: PathBuf,
    /// Parent directory of the manifest (root-relative).
    pub root_dir: PathBuf,
    /// Module name extracted from the manifest.
    pub name: String,
    /// Source files (relative to repo root) belonging to this module.
    pub source_files: Vec<PathBuf>,
}

/// Identify all modules in a `RepoWalk`.
///
/// Returns `Err` if a manifest has invalid syntax.
/// Manifests with no name field (e.g. workspace-root Cargo.toml) are skipped.
pub fn identify_modules(walk: &RepoWalk) -> Result<Vec<ProjectModule>> {
    let manifests = collect_manifests(walk);
    let mut modules = Vec::new();
    for (kind, manifest_rel) in &manifests {
        let root_dir = manifest_rel.parent().unwrap_or(manifest_rel).to_path_buf();
        let abs = walk.root.join(manifest_rel);
        let name = match try_parse_name(kind, &abs)? {
            Some(n) => n,
            None => continue, // workspace root or nameless — skip
        };
        let source_files = collect_sources(walk, &root_dir, kind, &manifests);
        modules.push(ProjectModule {
            kind: kind.clone(),
            manifest_path: manifest_rel.clone(),
            root_dir,
            name,
            source_files,
        });
    }
    Ok(modules)
}

/// Find all manifest files in the walk (relative paths).
fn collect_manifests(walk: &RepoWalk) -> Vec<(ModuleKind, PathBuf)> {
    let mut out = Vec::new();
    for f in &walk.files {
        let fname = f.path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        let kind = match fname {
            "Cargo.toml" => Some(ModuleKind::RustCrate),
            "package.json" => Some(ModuleKind::NpmPackage),
            "pyproject.toml" | "setup.py" => Some(ModuleKind::PythonPackage),
            "go.mod" => Some(ModuleKind::GoModule),
            _ => None,
        };
        if let Some(k) = kind {
            out.push((k, f.path.clone()));
        }
    }
    out
}

/// Try to extract the module name.
/// Returns `Ok(None)` if the file is valid but has no name (workspace root).
/// Returns `Err` if the file is syntactically invalid.
fn try_parse_name(kind: &ModuleKind, abs: &std::path::Path) -> Result<Option<String>> {
    let content = std::fs::read_to_string(abs)
        .map_err(|e| anyhow::anyhow!("read {}: {e}", abs.display()))?;
    match kind {
        ModuleKind::RustCrate => toml_name(&content, abs),
        ModuleKind::NpmPackage => json_name(&content, abs),
        ModuleKind::PythonPackage => python_name(&content, abs),
        ModuleKind::GoModule => go_name(&content, abs),
    }
}

fn toml_name(content: &str, path: &std::path::Path) -> Result<Option<String>> {
    let v: toml::Value = toml::from_str(content)
        .map_err(|e| anyhow::anyhow!("invalid TOML {}: {e}", path.display()))?;
    let name = v.get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(str::to_owned);
    Ok(name) // None when workspace-root (no [package])
}

fn json_name(content: &str, path: &std::path::Path) -> Result<Option<String>> {
    let v: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| anyhow::anyhow!("invalid JSON {}: {e}", path.display()))?;
    Ok(v["name"].as_str().map(str::to_owned))
}

fn python_name(content: &str, _path: &std::path::Path) -> Result<Option<String>> {
    // pyproject.toml: [project].name or [tool.poetry].name
    if let Ok(v) = toml::from_str::<toml::Value>(content) {
        if let Some(n) = v.get("project").and_then(|p| p.get("name")).and_then(|n| n.as_str()) {
            return Ok(Some(n.to_owned()));
        }
        let poetry_name = v.get("tool")
            .and_then(|t| t.get("poetry"))
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
            .map(str::to_owned);
        if poetry_name.is_some() {
            return Ok(poetry_name);
        }
    }
    // setup.py: best-effort line scan
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("name") && t.contains('=') {
            if let Some(v) = t.splitn(2, '=').nth(1) {
                let name = v.trim().trim_matches(|c| c == '\'' || c == '"' || c == ',');
                if !name.is_empty() {
                    return Ok(Some(name.to_owned()));
                }
            }
        }
    }
    Ok(None)
}

fn go_name(content: &str, _path: &std::path::Path) -> Result<Option<String>> {
    for line in content.lines() {
        let t = line.trim();
        if t.starts_with("module ") {
            let module = t["module ".len()..].trim();
            let name = module.split('/').last().unwrap_or(module).to_owned();
            return Ok(Some(name));
        }
    }
    Ok(None)
}

/// Collect source files under `module_root` matching the module's language(s).
fn collect_sources(
    walk: &RepoWalk,
    module_root: &PathBuf,
    kind: &ModuleKind,
    all_manifests: &[(ModuleKind, PathBuf)],
) -> Vec<PathBuf> {
    walk.files
        .iter()
        .filter(|f| {
            if !f.path.starts_with(module_root) {
                return false;
            }
            // Skip files inside a nested manifest's dir (ignore root-level manifests
            // whose parent is empty — they would match every path via starts_with(""))
            let is_nested = all_manifests.iter().any(|(_, m)| {
                let m_root = m.parent().unwrap_or(m);
                m_root != module_root
                    && !m_root.as_os_str().is_empty()
                    && f.path.starts_with(m_root)
            });
            if is_nested {
                return false;
            }
            matches!(
                (&f.language, kind),
                (Some(Language::Rust), ModuleKind::RustCrate)
                    | (Some(Language::TypeScript), ModuleKind::NpmPackage)
                    | (Some(Language::JavaScript), ModuleKind::NpmPackage)
                    | (Some(Language::Python), ModuleKind::PythonPackage)
                    | (Some(Language::Go), ModuleKind::GoModule)
            )
        })
        .map(|f| f.path.clone())
        .collect()
}

// Tests live in tests/identifier_tests.rs to keep this file ≤200 LOC.
