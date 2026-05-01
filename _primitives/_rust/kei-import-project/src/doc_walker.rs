//! Walk a repo root and collect documentation markdown file paths.
//!
//! Collects: README.md / README / readme.md at top level + every
//! docs/**/*.md (skipping _*.md, .git/, target/, node_modules/).

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Return ordered list of candidate markdown paths to extract skills from.
pub fn collect_doc_paths(repo_root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_readmes(repo_root, &mut out);
    collect_docs_dir(repo_root, &mut out);
    out
}

fn collect_readmes(root: &Path, out: &mut Vec<PathBuf>) {
    for name in &["README.md", "README", "readme.md"] {
        let candidate = root.join(name);
        if candidate.is_file() {
            out.push(candidate);
            break; // take first match only
        }
    }
}

fn collect_docs_dir(root: &Path, out: &mut Vec<PathBuf>) {
    let docs_root = root.join("docs");
    if !docs_root.is_dir() {
        return;
    }
    for entry in WalkDir::new(&docs_root)
        .follow_links(false)
        .into_iter()
        .flatten()
    {
        if should_skip(entry.path()) {
            continue;
        }
        if entry.file_type().is_file() && is_markdown(entry.path()) {
            out.push(entry.path().to_path_buf());
        }
    }
}

fn should_skip(path: &Path) -> bool {
    path.components().any(|c| {
        let s = c.as_os_str().to_string_lossy();
        matches!(s.as_ref(), ".git" | "target" | "node_modules")
    }) || path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('_'))
        .unwrap_or(false)
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("md"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn finds_readme_and_docs() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("README.md"), "# Hello").unwrap();
        let docs = dir.path().join("docs");
        fs::create_dir(&docs).unwrap();
        fs::write(docs.join("guide.md"), "## Guide\nbody").unwrap();
        fs::write(docs.join("_internal.md"), "skip").unwrap();

        let paths = collect_doc_paths(dir.path());
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().any(|p| p.ends_with("README.md")));
        assert!(paths.iter().any(|p| p.ends_with("guide.md")));
        assert!(!paths.iter().any(|p| p.ends_with("_internal.md")));
    }

    #[test]
    fn skips_target_dir() {
        let dir = TempDir::new().unwrap();
        let docs = dir.path().join("docs");
        fs::create_dir(&docs).unwrap();
        let target_docs = docs.join("target");
        fs::create_dir(&target_docs).unwrap();
        fs::write(target_docs.join("build.md"), "skip me").unwrap();
        fs::write(docs.join("real.md"), "## Real\nbody").unwrap();

        let paths = collect_doc_paths(dir.path());
        assert!(paths.iter().any(|p| p.ends_with("real.md")));
        assert!(!paths.iter().any(|p| p.to_string_lossy().contains("target")));
    }
}
