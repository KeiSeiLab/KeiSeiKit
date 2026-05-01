//! Patch-advisory: parses a unified-diff-like patch for file removals
//! or renames. Returns basenames the patch claims to remove.

use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn parse_removals(patch_file: &Path) -> HashSet<String> {
    let text = fs::read_to_string(patch_file).unwrap_or_default();
    let mut out = HashSet::new();
    for line in text.lines() {
        if let Some(stripped) = line.strip_prefix("--- a/") {
            // A `+++ /dev/null` on the next line would mean full removal;
            // we don't track across lines, so treat any "--- a/x" as POSSIBLY
            // touched. Conservative: we only add if `+++ /dev/null` appears
            // later somewhere in the file.
            if text.contains("+++ /dev/null") {
                add_basename(stripped, &mut out);
            }
        }
        // Also accept a lightweight header `# removed: path`
        if let Some(s) = line.strip_prefix("# removed: ") {
            add_basename(s.trim(), &mut out);
        }
    }
    out
}

fn add_basename(rel: &str, out: &mut HashSet<String>) {
    if let Some(name) = Path::new(rel).file_stem().and_then(|s| s.to_str()) {
        out.insert(name.to_lowercase());
    }
}
