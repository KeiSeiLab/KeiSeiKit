//! Graph resolver — indexes files then walks refs.

use regex::Regex;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub struct BrokenRef {
    pub source: String,
    pub line: usize,
    pub target: String,
    pub kind: String,
}

pub struct Graph {
    pub basenames: HashSet<String>,
    pub files: Vec<PathBuf>,
}

impl Graph {
    pub fn index(root: &Path) -> Self {
        let mut basenames = HashSet::new();
        let mut files = Vec::new();
        for e in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if e.file_type().is_file() {
                if let Some(stem) = e.path().file_stem().and_then(|s| s.to_str()) {
                    basenames.insert(stem.to_lowercase());
                }
                files.push(e.into_path());
            }
        }
        Self { basenames, files }
    }

    pub fn check(&self, root: &Path, removed: &HashSet<String>) -> Vec<BrokenRef> {
        let mut out = Vec::new();
        for file in &self.files {
            if file.extension().is_none_or(|e| e != "md") {
                continue;
            }
            out.extend(self.check_file(root, file, removed));
        }
        out
    }

    fn check_file(&self, root: &Path, file: &Path, removed: &HashSet<String>) -> Vec<BrokenRef> {
        let content = fs::read(file)
            .map(|b| String::from_utf8_lossy(&b).into_owned())
            .unwrap_or_default();
        let src = file
            .strip_prefix(root)
            .unwrap_or(file)
            .to_string_lossy()
            .into_owned();
        let mut out = Vec::new();
        for (ln, line) in content.lines().enumerate() {
            out.extend(scan_wikilinks(&src, ln + 1, line, &self.basenames, removed));
        }
        out
    }
}

fn scan_wikilinks(
    src: &str,
    line_no: usize,
    line: &str,
    index: &HashSet<String>,
    removed: &HashSet<String>,
) -> Vec<BrokenRef> {
    let rx = Regex::new(r"\[\[([^\]\|#]+?)(?:#[^\]]*)?(?:\|[^\]]*)?\]\]").expect("static regex");
    let mut out = Vec::new();
    for c in rx.captures_iter(line) {
        let target = c[1].trim().to_lowercase();
        let broken = !index.contains(&target) || removed.contains(&target);
        if broken {
            out.push(BrokenRef {
                source: src.to_string(),
                line: line_no,
                target,
                kind: "wikilink".to_string(),
            });
        }
    }
    out
}
