//! Secret-reference orphan detector.
//!
//! Reads env-var NAMES from `.env` files (never values), greps the kit
//! tree for usages, returns a `SecretsReport` with per-key usage counts
//! and orphan list. Constructor Pattern: pure read-side cube.

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretsReport {
    pub keys: Vec<KeyRow>,
    pub scanned_files: u64,
    pub env_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRow {
    pub name: String,
    pub source_env_file: String,
    pub usage_count: u64,
    /// Top 5 files where the key appears.
    pub usage_files: Vec<String>,
    pub orphan: bool,
}

const SKIP_DIRS: &[&str] = &["target", "node_modules", ".git", "_generated"];
const TEXT_EXTS: &[&str] = &["rs", "toml", "md", "sh", "py", "ts", "js", "yml", "yaml", "json"];

pub(crate) fn is_valid_key(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_uppercase() => {
            chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
        }
        _ => false,
    }
}

pub(crate) fn parse_env_file(path: &Path) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    let mut keys = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
        let Some(idx) = trimmed.find('=') else { continue; };
        let key = trimmed[..idx].trim();
        if is_valid_key(key) { keys.push(key.to_string()); }
    }
    Ok(keys)
}

fn is_text_file(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).map_or(false, |ext| TEXT_EXTS.contains(&ext))
}

fn word_re(key: &str) -> Result<Regex> {
    Ok(Regex::new(&format!(r"\b{}\b", regex::escape(key)))?)
}

/// Scan `scan_root`, returning scanned_files count and per-key (count, files) map.
pub(crate) fn scan_usages(
    keys: &[String],
    scan_root: &Path,
) -> Result<(u64, BTreeMap<String, (u64, Vec<String>)>)> {
    let patterns: Vec<(String, Regex)> = keys
        .iter()
        .map(|k| Ok((k.clone(), word_re(k)?)))
        .collect::<Result<_>>()?;
    let mut counts: BTreeMap<String, (u64, Vec<String>)> = BTreeMap::new();
    for k in keys { counts.insert(k.clone(), (0, Vec::new())); }
    let mut scanned = 0u64;
    for entry in WalkDir::new(scan_root).follow_links(false)
        .into_iter()
        .filter_entry(|e| !SKIP_DIRS.contains(&e.file_name().to_string_lossy().as_ref()))
        .flatten()
    {
        if !entry.file_type().is_file() || !is_text_file(entry.path()) { continue; }
        let Ok(content) = std::fs::read_to_string(entry.path()) else { continue; };
        scanned += 1;
        let rel = entry.path().strip_prefix(scan_root).unwrap_or(entry.path())
            .to_string_lossy().to_string();
        for (key, re) in &patterns {
            if re.is_match(&content) {
                let e = counts.get_mut(key).expect("key present");
                e.0 += 1;
                if e.1.len() < 5 { e.1.push(rel.clone()); }
            }
        }
    }
    Ok((scanned, counts))
}

/// Build a `SecretsReport`. Pure: no side effects beyond file reads.
pub fn compute_secrets_report(env_paths: &[PathBuf], scan_root: &Path) -> Result<SecretsReport> {
    let mut all_keys: Vec<(String, String)> = Vec::new();
    let mut env_file_labels: Vec<String> = Vec::new();
    for ep in env_paths {
        let label = ep.to_string_lossy().to_string();
        env_file_labels.push(label.clone());
        for k in parse_env_file(ep).unwrap_or_default() { all_keys.push((k, label.clone())); }
    }
    let unique_keys: Vec<String> = {
        let mut seen = std::collections::HashSet::new();
        all_keys.iter().filter(|(k, _)| seen.insert(k.clone())).map(|(k, _)| k.clone()).collect()
    };
    let (scanned_files, counts) = scan_usages(&unique_keys, scan_root)?;
    let mut rows: Vec<KeyRow> = all_keys.into_iter().map(|(name, source_env_file)| {
        let (usage_count, usage_files) = counts.get(&name).cloned().unwrap_or_default();
        KeyRow { orphan: usage_count == 0, name, source_env_file, usage_count, usage_files }
    }).collect();
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(SecretsReport { keys: rows, scanned_files, env_files: env_file_labels })
}

/// Render a `SecretsReport` as ASCII text.
pub fn render_ascii(r: &SecretsReport) -> String {
    use std::fmt::Write as FmtWrite;
    let mut out = String::new();
    let mut by_file: BTreeMap<&str, Vec<&KeyRow>> = BTreeMap::new();
    for row in &r.keys { by_file.entry(row.source_env_file.as_str()).or_default().push(row); }
    for (file, rows) in &by_file {
        let orphan_count = rows.iter().filter(|r| r.orphan).count();
        let _ = writeln!(out, "[Secrets — {} ({} keys, {} orphan)]", file, rows.len(), orphan_count);
        for row in rows.iter() { render_row(&mut out, row); }
    }
    let total_orphans = r.keys.iter().filter(|k| k.orphan).count();
    let _ = writeln!(out, "Total: {} keys across {} env files, {} orphan",
        r.keys.len(), r.env_files.len(), total_orphans);
    out
}

fn render_row(out: &mut String, row: &KeyRow) {
    use std::fmt::Write as FmtWrite;
    if row.orphan {
        let _ = writeln!(out, "  {:<35} *ORPHAN*  0 refs   — candidate for removal", row.name);
        return;
    }
    let files_str = row.usage_files.join(", ");
    let extra = if row.usage_count > row.usage_files.len() as u64 {
        format!(", +{} more", row.usage_count - row.usage_files.len() as u64)
    } else { String::new() };
    let _ = writeln!(out, "  {:<35} {:>4} refs   ({}{})", row.name, row.usage_count, files_str, extra);
}

#[cfg(test)]
#[path = "secrets_tests.rs"]
mod tests;
