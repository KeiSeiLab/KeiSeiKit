//! walker — traverse a repo root, classify files by language, skip noise dirs.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MAX_FILE_BYTES: u64 = 10 * 1024 * 1024; // 10 MB

static IGNORED_DIRS: &[&str] = &[
    "target", "node_modules", ".git", "dist", "build", ".venv", "__pycache__",
    ".tox", ".mypy_cache", "coverage", ".next", ".nuxt", "out",
];

/// Language detected from file extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    Go,
    Markdown,
    Toml,
    Json,
    Yaml,
    Sql,
    Shell,
    Other,
}

/// A single file entry from `walk_repo`.
pub struct FileEntry {
    /// Path relative to the repo root.
    pub path: PathBuf,
    /// Language detected by extension; `None` means binary/unknown/oversized.
    pub language: Option<Language>,
    /// File size in bytes.
    pub size_bytes: u64,
}

/// Result of walking a repository root.
pub struct RepoWalk {
    pub root: PathBuf,
    pub files: Vec<FileEntry>,
}

/// Walk `root`, ignoring noise directories and files >10 MB.
pub fn walk_repo(root: &Path) -> Result<RepoWalk> {
    anyhow::ensure!(root.exists(), "root does not exist: {}", root.display());
    anyhow::ensure!(root.is_dir(), "root is not a directory: {}", root.display());

    let mut files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false).into_iter() {
        let entry = entry?;
        if should_skip_dir(&entry) {
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let abs = entry.path();
        let rel = abs.strip_prefix(root).unwrap_or(abs).to_path_buf();
        let size_bytes = abs.metadata().map(|m| m.len()).unwrap_or(0);
        let language = if size_bytes > MAX_FILE_BYTES {
            None
        } else {
            detect_language(abs)
        };
        files.push(FileEntry { path: rel, language, size_bytes });
    }
    Ok(RepoWalk { root: root.to_path_buf(), files })
}

fn should_skip_dir(entry: &walkdir::DirEntry) -> bool {
    if entry.file_type().is_dir() {
        if let Some(name) = entry.file_name().to_str() {
            return IGNORED_DIRS.contains(&name);
        }
    }
    // Also skip files whose ancestor component is an ignored dir.
    for component in entry.path().components() {
        if let std::path::Component::Normal(s) = component {
            if let Some(name) = s.to_str() {
                if IGNORED_DIRS.contains(&name) {
                    return true;
                }
            }
        }
    }
    false
}

fn detect_language(path: &Path) -> Option<Language> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    let lang = match ext.as_str() {
        "rs" => Language::Rust,
        "ts" | "tsx" => Language::TypeScript,
        "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
        "py" => Language::Python,
        "go" => Language::Go,
        "md" | "mdx" => Language::Markdown,
        "toml" => Language::Toml,
        "json" => Language::Json,
        "yaml" | "yml" => Language::Yaml,
        "sql" => Language::Sql,
        "sh" | "bash" | "zsh" => Language::Shell,
        _ => Language::Other,
    };
    Some(lang)
}

// ─────────────────────────── tests ──────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn mk_file(dir: &Path, rel: &str, content: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(p, content).unwrap();
    }

    #[test]
    fn happy_path_five_files() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        mk_file(root, "src/main.rs", "fn main() {}");
        mk_file(root, "src/lib.rs", "pub fn foo() {}");
        mk_file(root, "Cargo.toml", "[package]");
        mk_file(root, "README.md", "# Hello");
        mk_file(root, "script.sh", "#!/bin/bash");

        let walk = walk_repo(root).unwrap();
        assert_eq!(walk.files.len(), 5);
    }

    #[test]
    fn ignores_target_and_git() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        mk_file(root, "src/main.rs", "fn main() {}");
        mk_file(root, "target/release/binary", "binary");
        mk_file(root, ".git/config", "config");
        mk_file(root, "node_modules/pkg/index.js", "module");

        let walk = walk_repo(root).unwrap();
        assert_eq!(walk.files.len(), 1);
        assert_eq!(walk.files[0].path, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn detects_language_variants() {
        let cases: &[(&str, Language)] = &[
            ("a.rs", Language::Rust),
            ("b.ts", Language::TypeScript),
            ("c.tsx", Language::TypeScript),
            ("d.js", Language::JavaScript),
            ("e.py", Language::Python),
            ("f.go", Language::Go),
            ("g.md", Language::Markdown),
            ("h.toml", Language::Toml),
            ("i.json", Language::Json),
            ("j.yaml", Language::Yaml),
            ("k.sql", Language::Sql),
            ("l.sh", Language::Shell),
        ];
        for (filename, expected) in cases {
            let result = detect_language(Path::new(filename));
            assert_eq!(result.as_ref(), Some(expected), "file: {filename}");
        }
    }

    #[test]
    fn nonexistent_root_returns_err() {
        let result = walk_repo(Path::new("/nonexistent/path/abc123"));
        assert!(result.is_err());
    }
}
