//! module_source — lightweight source-file bundle consumed by the trait matcher.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.
//! A1.2 will adapt RepoWalk → ModuleSource; until then this is standalone.

use std::io;
use std::path::{Path, PathBuf};

/// A named module with its pre-loaded Rust source files.
///
/// Each tuple is `(path-relative-to-src, full-file-contents)`.
/// Non-Rust files and unreadable files are silently omitted.
pub struct ModuleSource {
    pub name: String,
    pub source_files: Vec<(PathBuf, String)>,
}

impl ModuleSource {
    /// Build from in-memory content — used in unit tests.
    pub fn from_content(
        name: impl Into<String>,
        files: Vec<(PathBuf, String)>,
    ) -> Self {
        Self { name: name.into(), source_files: files }
    }

    /// Walk `dir`, read every `.rs` file, return a `ModuleSource`.
    ///
    /// Files that cannot be read are silently skipped.
    /// Non-`.rs` files are ignored (the matcher only uses Rust source).
    pub fn from_dir(name: impl Into<String>, dir: &Path) -> io::Result<Self> {
        let name = name.into();
        let source_files = collect_rs_files(dir)?;
        Ok(Self { name, source_files })
    }
}

/// Recursively collect all `.rs` files under `dir`.
fn collect_rs_files(dir: &Path) -> io::Result<Vec<(PathBuf, String)>> {
    let mut out = Vec::new();
    visit(dir, dir, &mut out);
    Ok(out)
}

fn visit(root: &Path, current: &Path, out: &mut Vec<(PathBuf, String)>) {
    let entries = match std::fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit(root, &path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let rel = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
            if let Ok(content) = std::fs::read_to_string(&path) {
                out.push((rel, content));
            }
        }
    }
}

// ─────────────────────────── tests ──────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn mk(dir: &Path, rel: &str, content: &str) {
        let p = dir.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(p, content).unwrap();
    }

    #[test]
    fn from_dir_reads_rs_files() {
        let tmp = TempDir::new().unwrap();
        mk(tmp.path(), "src/lib.rs", "pub fn foo() {}");
        mk(tmp.path(), "src/main.rs", "fn main() {}");
        mk(tmp.path(), "README.md", "# hello");

        let ms = ModuleSource::from_dir("my-crate", tmp.path()).unwrap();
        assert_eq!(ms.name, "my-crate");
        assert_eq!(ms.source_files.len(), 2);
    }

    #[test]
    fn from_dir_skips_non_rs() {
        let tmp = TempDir::new().unwrap();
        mk(tmp.path(), "Cargo.toml", "[package]");
        mk(tmp.path(), "src/lib.rs", "");

        let ms = ModuleSource::from_dir("crate", tmp.path()).unwrap();
        assert_eq!(ms.source_files.len(), 1);
    }

    #[test]
    fn from_content_builds_directly() {
        let files = vec![(PathBuf::from("lib.rs"), "fn foo() {}".into())];
        let ms = ModuleSource::from_content("synthetic", files);
        assert_eq!(ms.name, "synthetic");
        assert_eq!(ms.source_files.len(), 1);
    }
}
