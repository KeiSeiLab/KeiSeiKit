//! Skill scanner — walks `<kit-root>/skills/*/SKILL.md`.
//!
//! Constructor Pattern: this cube knows the SKILL.md convention only.
//! Body bytes = raw markdown; name = first H1 line stripped of leading
//! `# ` (fallback: directory name); caps = `md`.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use super::{Found, Scanner};
use crate::block::BlockType;

/// `<kit-root>/skills/<name>/SKILL.md` adapter.
pub struct SkillScanner;

impl Scanner for SkillScanner {
    fn scan(&self, root: &Path) -> Result<Vec<Found>> {
        let skills_root = root.join("skills");
        if !skills_root.is_dir() {
            return Ok(Vec::new());
        }
        let mut found = Vec::new();
        for entry in fs::read_dir(&skills_root)? {
            let entry = entry?;
            let skill_dir = entry.path();
            if !skill_dir.is_dir() {
                continue;
            }
            if let Some(f) = scan_one_skill(&skill_dir)? {
                found.push(f);
            }
        }
        found.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(found)
    }
}

/// Scan a single skill directory directly (used by `register-skill`
/// shorthand which receives the skill dir, not the kit root).
pub fn scan_one_skill(skill_dir: &Path) -> Result<Option<Found>> {
    let skill_md = skill_dir.join("SKILL.md");
    if !skill_md.is_file() {
        return Ok(None);
    }
    let body = match fs::read(&skill_md) {
        Ok(b) => b,
        Err(_) => return Ok(None),
    };
    let dir_name = skill_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    let name = extract_h1_title(&body).unwrap_or(dir_name);
    let path = canonical_str(&skill_md);
    Ok(Some(Found {
        block_type: BlockType::Skill,
        name,
        path,
        body,
        caps: "md".to_string(),
    }))
}

/// Extract the first H1 line (`# Title`) from markdown bytes. Returns None
/// if no H1 is present in the first 200 lines.
fn extract_h1_title(body: &[u8]) -> Option<String> {
    let txt = std::str::from_utf8(body).ok()?;
    for line in txt.lines().take(200) {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            let title = rest.trim().trim_end_matches('#').trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

fn canonical_str(p: &Path) -> String {
    p.canonicalize()
        .unwrap_or_else(|_| PathBuf::from(p))
        .to_string_lossy()
        .to_string()
}
